<template>
  <div class="calendar">
    <img src="/vite.svg" alt="Calendar" style="width:50px;"/>
    <img :src="vueimage" style="width:50px;"/>
    <div class="month-grid">
      <div
        v-for="day in generatedMonthDays"
        :key="day"
        class="day-cell"
      >
        <div class="day-header">
          <span>{{ day }}</span>
          <button @click="openNewTask(day)">+</button>
        </div>
        <ul>
          <li v-for="task in tasks[day] || []" :key="task.id">
            {{ task.text }}
            <button @click="editTask(day, task)">✎</button>
            <button @click="deleteTask(task.id)">🗑</button>
          </li>
        </ul>
      </div>
    </div>

    <div v-if="showDialog" class="dialog">
      <h3>{{ editingTask ? "Edit" : "Add" }} Task</h3>
      <label>Date:</label>
      <input v-model="selectedDate" placeholder="YYYY-MM-DD" />
      <label>Task:</label>
      <input v-model="taskText" />
      <button @click="submitTask">Save</button>
      <button @click="cancel">Cancel</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

import vueimage from "../assets/vue.svg"

interface Task {
  id: number;
  text: string;
}
/*
interface Day {
  date: string;
  tasks: Task[];
}
*/

const backendPort = ref(8080); // Replace with your backend port
const tasks = ref({} as any);
const selectedDate = ref('' as String);
const taskText = ref('');
const editingTask = ref<Task | null>(null);
const showDialog = ref(false);

// Generate 30-day dummy month view
const generatedMonthDays = Array.from({ length: 30 }, (_, i) => {
  const day = new Date();
  day.setDate(1 + i);
  return day.toISOString().split("T")[0]; // YYYY-MM-DD
});

async function fetchMonth() {
  const url = `http://localhost:${backendPort.value}/api/calendar/month`;
  console.log("Get URL:", url);
  const res = await fetch(url)
  const data = await res.json();
  tasks.value = {};
  for (const day of data.days) {
    tasks.value[day.date] = day.tasks;
  }
}

function openNewTask(date:String) {
  selectedDate.value = date;
  taskText.value = '';
  editingTask.value = null;
  showDialog.value = true;
}

function editTask(date:String, task:Task) {
  selectedDate.value = date;
  taskText.value = task.text;
  editingTask.value = task;
  showDialog.value = true;
}

async function deleteTask(id:Number) {
  const url = `http://localhost:${backendPort.value}/api/calendar/task/${id}`;
  console.log("Deleting task at URL:", url);
  await fetch(url, {
    method: 'DELETE'
  });
  fetchMonth();
}

async function submitTask() {
  if (editingTask.value) {
  const url = `http://localhost:${backendPort.value}/api/calendar/task/${editingTask.value.id}`;
  console.log("Edit URL:", url);
  await fetch(url, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ date: selectedDate.value, text: taskText.value }),
    });
  } else {
    const url = `http://localhost:${backendPort.value}/api/calendar/task`;
    console.log("Post URL:", url);
  await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ date: selectedDate.value, text: taskText.value }),
    });
  }
  showDialog.value = false;
  fetchMonth();
}

function cancel() {
  showDialog.value = false;
}

onMounted(async () => {
  /*
  const config  = await invoke("get_config");
  console.log("Ports from Rust:", config[0], config[1]);
  backendPort.value = config[1];
  console.log("Backend port set to:", backendPort.value);
  */
  await fetchMonth()
}
);
</script>

<style>
.calendar {
  padding: 20px;
}
.month-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
}
.day-cell {
  border: 1px solid #ccc;
  padding: 5px;
}
.day-header {
  display: flex;
  justify-content: space-between;
}
.dialog {
  position: fixed;
  top: 20%;
  left: 30%;
  background: #fff;
  padding: 20px;
  border: 1px solid #000;
  min-width: 300px;
}
</style>