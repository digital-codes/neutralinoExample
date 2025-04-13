const fs = require('fs');
const path = require('path');

// Path to your index.html — change if needed
const htmlPath = path.join(__dirname, '../vue-src/', 'index.html');

// Correct line to insert
const correctScriptTag = '<script src="%PUBLIC_URL%/__neutralino_globals.js"></script>';

// Read the file
fs.readFile(htmlPath, 'utf-8', (err, html) => {
  if (err) {
    console.error(`Error reading ${htmlPath}:`, err);
    return;
  }

  // Regex to detect incorrect script tag with localhost and port
  const badScriptRegex = /<script\s+src="http:\/\/localhost:\d+\/__neutralino_globals\.js"><\/script>/;

  if (badScriptRegex.test(html)) {
    console.log('Bad script tag found. Patching...');
    const updatedHtml = html
      .replace(badScriptRegex, '') // Remove the bad line
      .replace('</head>', `  ${correctScriptTag}\n</head>`); // Insert correct line before </head>

    // Write back the updated file
    fs.writeFile(htmlPath, updatedHtml, 'utf-8', (err) => {
      if (err) {
        console.error(`Error writing patched HTML to ${htmlPath}:`, err);
      } else {
        console.log('Successfully patched index.html');
      }
    });
  } else {
    console.log('No incorrect script tag found. No changes made.');
  }
});
