const asar = require('asar');
const path = require('path');

// Define the path to your resources.neu (ASAR) file.
const asarFilePath = path.join(__dirname, '../dist/vue/resources.neu');

// Define the destination directory for extraction.
const outputDir = path.join(__dirname, 'extracted_resources');

// Extract all files in the ASAR archive.
asar.extractAll(asarFilePath, outputDir);

console.log('Extraction complete.');
