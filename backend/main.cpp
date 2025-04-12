#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/strand.hpp>
#include <boost/config.hpp>
#include <boost/program_options.hpp>
#include <json/json.h>

#include <cstdlib>
#include <iostream>
#include <memory>
#include <string>
#include <thread>
#include <map>
#include <vector>
#include <sstream>
#include <regex>


namespace po = boost::program_options;
namespace beast = boost::beast;
namespace http = beast::http;
namespace net = boost::asio;
using tcp = net::ip::tcp;

struct Task
{
    int id;
    std::string text;
};

std::map<std::string, std::vector<Task>> calendar_db;
int next_id = 1;

std::string serializeMonth()
{
    Json::Value days(Json::arrayValue);
    for (const auto &[date, tasks] : calendar_db)
    {
        Json::Value day;
        day["date"] = date;
        for (const auto &task : tasks)
        {
            Json::Value t;
            t["id"] = task.id;
            t["text"] = task.text;
            day["tasks"].append(t);
        }
        days.append(day);
    }
    Json::Value result;
    result["days"] = days;
    Json::StreamWriterBuilder writer;
    return Json::writeString(writer, result);
}

std::string addTask(const std::string &body)
{
    Json::CharReaderBuilder reader;
    Json::Value data;
    std::string errs;
    std::istringstream s(body);
    std::string date, text;

    if (Json::parseFromStream(reader, s, &data, &errs))
    {
        date = data["date"].asString();
        text = data["text"].asString();
        Task task{next_id++, text};
        calendar_db[date].push_back(task);

        Json::Value result;
        result["id"] = task.id;
        result["text"] = task.text;
        Json::StreamWriterBuilder writer;
        return Json::writeString(writer, result);
    }
    return "{}";
}

std::string updateTask(int id, const std::string &body)
{
    Json::CharReaderBuilder reader;
    Json::Value data;
    std::string errs;
    std::istringstream s(body);

    if (Json::parseFromStream(reader, s, &data, &errs))
    {
        std::string date = data["date"].asString();
        std::string newText = data["text"].asString();
        for (auto &task : calendar_db[date])
        {
            if (task.id == id)
            {
                task.text = newText;
                break;
            }
        }
        return "OK";
    }
    return "Failed";
}

std::string deleteTask(int id)
{
    for (auto &[_, tasks] : calendar_db)
    {
        tasks.erase(std::remove_if(tasks.begin(), tasks.end(),
                                   [id](const Task &t)
                                   { return t.id == id; }),
                    tasks.end());
    }
    return "OK";
}

void handle_request(http::request<http::string_body> req,
                    http::response<http::string_body> &res)
{
    res.set(http::field::content_type, "application/json");
    res.set(http::field::access_control_allow_origin, "*");
    res.set(http::field::access_control_allow_methods, "GET, POST, PUT, DELETE, OPTIONS");
    res.set(http::field::access_control_allow_headers, "Content-Type");

    std::string target = std::string(req.target());
    std::smatch match;

    if (req.method() == http::verb::options) {
        res.result(http::status::no_content);
        return;
    }
    
    if (req.method() == http::verb::get && target == "/api/calendar/month")
    {
        res.result(http::status::ok);
        res.body() = serializeMonth();
    }
    else if (req.method() == http::verb::post && target == "/api/calendar/task")
    {
        res.result(http::status::ok);
        res.body() = addTask(req.body());
    }
    else if (req.method() == http::verb::put &&
             std::regex_match(target, match, std::regex("/api/calendar/task/(\\d+)")))
    {
        int id = std::stoi(match[1].str());
        res.result(http::status::ok);
        res.body() = updateTask(id, req.body());
    }
    else if (req.method() == http::verb::delete_ &&
             std::regex_match(target, match, std::regex("/api/calendar/task/(\\d+)")))
    {
        int id = std::stoi(match[1].str());
        res.result(http::status::ok);
        res.body() = deleteTask(id);
    }
    else
    {
        res = http::response<http::string_body>(http::status::not_found, req.version());
        res.set(http::field::content_type, "text/plain");
        res.body() = "Not Found";
    }

    res.prepare_payload();
}

void do_session(tcp::socket socket)
{
    try
    {
        beast::flat_buffer buffer;
        http::request<http::string_body> req;
        http::read(socket, buffer, req);

        http::response<http::string_body> res;
        handle_request(std::move(req), res);

        http::write(socket, res);
    }
    catch (std::exception const &e)
    {
        std::cerr << "Error: " << e.what() << std::endl;
    }
}


int main(int argc, char *argv[])
{
    try
    {
        int local_port = 8080;
        int gui_port = 3000;

        // Define and parse command-line options
        po::options_description desc("Allowed options");
        desc.add_options()
            ("help,h", "Show help message")
            ("local-port,l", po::value<int>(&local_port)->default_value(8080), "Set local server port")
            ("gui-port,g", po::value<int>(&gui_port)->default_value(3000), "Set GUI server port");

        po::variables_map vm;
        po::store(po::parse_command_line(argc, argv, desc), vm);
        po::notify(vm);

        if (vm.count("help"))
        {
            std::cout << desc << std::endl;
            return 0;
        }

        net::io_context ioc{1};
        tcp::acceptor acceptor{ioc, {tcp::v4(), static_cast<unsigned short>(local_port)}};

        std::cout << "Boost HTTP Server running on http://localhost:" << local_port << std::endl;
        std::cout << "GUI Server expected on port " << gui_port << std::endl;

        for (;;)
        {
            tcp::socket socket{ioc};
            acceptor.accept(socket);
            std::thread([sock = std::move(socket)]() mutable
                        { do_session(std::move(sock)); })
                .detach();
        }
    }
    catch (std::exception const &e)
    {
        std::cerr << "Fatal error: " << e.what() << std::endl;
        return EXIT_FAILURE;
    }
}
