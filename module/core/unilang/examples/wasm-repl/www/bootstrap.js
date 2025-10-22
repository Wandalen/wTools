// Bootstrap script for UniLang WebAssembly REPL

let wasm_module;
let repl_instance;

async function init() {
  try {
    // Import the WASM module
    wasm_module = await import('../pkg/unilang_wasm_repl.js');
    
    // Create a new REPL instance
    repl_instance = new wasm_module.UniLangWasmRepl();
    
    console.log('UniLang WASM REPL initialized successfully! 🚀');
    
    // Set up event listeners
    setupEventListeners();
    
    // Show initial help
    appendOutput('system', 'UniLang WebAssembly REPL loaded successfully! ✨');
    
  } catch (error) {
    console.error('Failed to initialize WASM module:', error);
    appendOutput('error', `Failed to load WebAssembly module: ${error.message}`);
  }
}

function setupEventListeners() {
  const commandInput = document.getElementById('command-input');
  const executeBtn = document.getElementById('execute-btn');
  const loadCommandsBtn = document.getElementById('load-commands-btn');
  const jsonCommandsArea = document.getElementById('json-commands');
  
  // Execute command on button click
  executeBtn.addEventListener('click', executeCommand);
  
  // Execute command on Enter key
  commandInput.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
      executeCommand();
    }
  });
  
  // Load custom commands
  loadCommandsBtn.addEventListener('click', loadCustomCommands);
  
  // Command history (simple implementation)
  let commandHistory = [];
  let historyIndex = -1;
  
  commandInput.addEventListener('keydown', (event) => {
    if (event.key === 'ArrowUp') {
      event.preventDefault();
      if (historyIndex < commandHistory.length - 1) {
        historyIndex++;
        commandInput.value = commandHistory[commandHistory.length - 1 - historyIndex];
      }
    } else if (event.key === 'ArrowDown') {
      event.preventDefault();
      if (historyIndex > 0) {
        historyIndex--;
        commandInput.value = commandHistory[commandHistory.length - 1 - historyIndex];
      } else if (historyIndex === 0) {
        historyIndex = -1;
        commandInput.value = '';
      }
    } else if (event.key === 'Enter') {
      // Add command to history
      const command = commandInput.value.trim();
      if (command && commandHistory[commandHistory.length - 1] !== command) {
        commandHistory.push(command);
        // Keep history size reasonable
        if (commandHistory.length > 100) {
          commandHistory.shift();
        }
      }
      historyIndex = -1;
    }
  });
}

function executeCommand() {
  const commandInput = document.getElementById('command-input');
  const command = commandInput.value.trim();
  
  if (!command) return;
  
  if (!repl_instance) {
    appendOutput('error', 'REPL not initialized. Please refresh the page.');
    return;
  }
  
  // Show the command being executed
  appendOutput('command', `> ${command}`);
  
  try {
    // Execute the command through WASM
    const result = repl_instance.execute_command(command);
    
    // Show the result
    if (result.startsWith('❌')) {
      appendOutput('error', result);
    } else {
      appendOutput('success', result);
    }
    
  } catch (error) {
    console.error('Command execution error:', error);
    appendOutput('error', `Execution error: ${error.message}`);
  }
  
  // Clear the input
  commandInput.value = '';
}

function loadCustomCommands() {
  const jsonCommandsArea = document.getElementById('json-commands');
  const jsonText = jsonCommandsArea.value.trim();
  
  if (!jsonText) {
    appendOutput('error', 'Please enter JSON command definitions');
    return;
  }
  
  if (!repl_instance) {
    appendOutput('error', 'REPL not initialized. Please refresh the page.');
    return;
  }
  
  try {
    const result = repl_instance.load_commands_json(jsonText);
    
    if (result.startsWith('❌')) {
      appendOutput('error', result);
    } else {
      appendOutput('success', result);
      jsonCommandsArea.value = ''; // Clear on success
    }
    
  } catch (error) {
    console.error('Command loading error:', error);
    appendOutput('error', `Loading error: ${error.message}`);
  }
}

function appendOutput(type, text) {
  const outputDiv = document.getElementById('output');
  const lineDiv = document.createElement('div');
  lineDiv.className = `command-line`;
  
  const contentDiv = document.createElement('div');
  
  switch (type) {
    case 'command':
      contentDiv.className = 'command-input';
      contentDiv.textContent = text;
      break;
    case 'success':
      contentDiv.className = 'command-output';
      contentDiv.textContent = text;
      break;
    case 'error':
      contentDiv.className = 'command-error';
      contentDiv.textContent = text;
      break;
    case 'system':
      contentDiv.className = 'command-output';
      contentDiv.style.color = '#68d391';
      contentDiv.textContent = text;
      break;
  }
  
  lineDiv.appendChild(contentDiv);
  outputDiv.appendChild(lineDiv);
  
  // Auto-scroll to bottom
  outputDiv.scrollTop = outputDiv.scrollHeight;
}

// Utility function to demonstrate command loading
function loadExampleCommands() {
  const exampleCommands = {
    "commands": [
      {
        "name": "greet",
        "namespace": ["demo"],
        "hint": "Greet someone",
        "description": "A friendly greeting command",
        "arguments": [
          {
            "name": "name",
            "kind": "String",
            "hint": "Person's name",
            "description": "The name of the person to greet",
            "properties": {}
          }
        ],
        "properties": {},
        "routine": "demo_greet_routine"
      }
    ]
  };
  
  document.getElementById('json-commands').value = JSON.stringify(exampleCommands, null, 2);
}

// Add a helper button for example commands
document.addEventListener('DOMContentLoaded', () => {
  const sidebar = document.querySelector('.sidebar');
  
  const exampleSection = document.createElement('div');
  exampleSection.className = 'section';
  exampleSection.innerHTML = `
    <h3>📋 Example</h3>
    <button id="load-example-btn" style="
      width: 100%;
      padding: 10px;
      background: #4a5568;
      color: #f7fafc;
      border: 1px solid #718096;
      border-radius: 8px;
      cursor: pointer;
    ">Load Example Commands</button>
  `;
  
  sidebar.appendChild(exampleSection);
  
  document.getElementById('load-example-btn').addEventListener('click', loadExampleCommands);
  
  // Initialize the WASM module
  init();
});

// Export for debugging
window.UniLangWASM = {
  repl_instance,
  executeCommand,
  loadCustomCommands,
  appendOutput
};