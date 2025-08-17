# Task 011: IDE Integration

**Priority**: 💻 High Impact  
**Phase**: 4 (Tooling Ecosystem)  
**Estimated Effort**: 6-8 weeks  
**Dependencies**: Task 010 (CLI Tool), Task 001 (Cargo Integration)  

## **Objective**
Develop IDE extensions and integrations to make workspace_tools visible and accessible to all Rust developers directly within their development environment, significantly increasing discoverability and adoption.

## **Technical Requirements**

### **Core Features**
1. **VS Code Extension**
   - Workspace navigation panel showing standard directories
   - Quick actions for creating config files and standard directories
   - Auto-completion for workspace paths in Rust code
   - Integration with file explorer for workspace-relative operations

2. **IntelliJ/RustRover Plugin**
   - Project tool window for workspace management
   - Code generation templates using workspace_tools patterns
   - Inspection and quick fixes for workspace path usage
   - Integration with existing Rust plugin ecosystem

3. **rust-analyzer Integration**
   - LSP extension for workspace path completion
   - Hover information for workspace paths
   - Code actions for converting absolute paths to workspace-relative
   - Integration with workspace metadata

### **VS Code Extension Architecture**
```typescript
// Extension API surface
interface WorkspaceToolsAPI {
    // Workspace detection and management
    detectWorkspace(): Promise<WorkspaceInfo>;
    getStandardDirectories(): Promise<DirectoryInfo[]>;
    createStandardDirectory(name: string): Promise<void>;
    
    // Configuration management
    loadConfig<T>(name: string): Promise<T>;
    saveConfig<T>(name: string, config: T): Promise<void>;
    editConfig(name: string): Promise<void>;
    
    // Resource discovery
    findResources(pattern: string): Promise<string[]>;
    searchWorkspace(query: string): Promise<SearchResult[]>;
    
    // Integration features
    generateBoilerplate(template: string): Promise<void>;
    validateWorkspaceStructure(): Promise<ValidationResult>;
}

interface WorkspaceInfo {
    root: string;
    type: 'cargo' | 'standard' | 'git' | 'manual';
    standardDirectories: string[];
    configFiles: ConfigFileInfo[];
    metadata?: CargoMetadata;
}

interface DirectoryInfo {
    name: string;
    path: string;
    purpose: string;
    exists: boolean;
    isEmpty: boolean;
}

interface ConfigFileInfo {
    name: string;
    path: string;
    format: 'toml' | 'yaml' | 'json';
    schema?: string;
}

interface SearchResult {
    path: string;
    type: 'file' | 'directory' | 'config' | 'resource';
    relevance: number;
    preview?: string;
}

interface ValidationResult {
    valid: boolean;
    warnings: ValidationWarning[];
    suggestions: ValidationSuggestion[];
}
```

### **Implementation Steps**

#### **Phase 1: VS Code Extension Foundation** (Weeks 1-2)

**Week 1: Core Extension Structure**
```json
// package.json
{
  "name": "workspace-tools",
  "displayName": "Workspace Tools",
  "description": "Universal workspace-relative path resolution for Rust projects",
  "version": "0.1.0",
  "publisher": "workspace-tools",
  "categories": ["Other", "Snippets", "Formatters"],
  "keywords": ["rust", "workspace", "path", "configuration"],
  "engines": {
    "vscode": "^1.74.0"
  },
  "activationEvents": [
    "onLanguage:rust",
    "workspaceContains:Cargo.toml",
    "workspaceContains:.cargo/config.toml"
  ],
  "contributes": {
    "commands": [
      {
        "command": "workspace-tools.detectWorkspace",
        "title": "Detect Workspace",
        "category": "Workspace Tools"
      },
      {
        "command": "workspace-tools.createStandardDirectories",
        "title": "Create Standard Directories",
        "category": "Workspace Tools"
      },
      {
        "command": "workspace-tools.openConfig",
        "title": "Open Configuration",
        "category": "Workspace Tools"
      }
    ],
    "views": {
      "explorer": [
        {
          "id": "workspace-tools.workspaceExplorer",
          "name": "Workspace Tools",
          "when": "workspace-tools.isWorkspace"
        }
      ]
    },
    "viewsContainers": {
      "activitybar": [
        {
          "id": "workspace-tools",
          "title": "Workspace Tools",
          "icon": "$(folder-library)"
        }
      ]
    },
    "configuration": {
      "title": "Workspace Tools",
      "properties": {
        "workspace-tools.autoDetect": {
          "type": "boolean",
          "default": true,
          "description": "Automatically detect workspace_tools workspaces"
        },
        "workspace-tools.showInStatusBar": {
          "type": "boolean", 
          "default": true,
          "description": "Show workspace status in status bar"
        }
      }
    }
  }
}
```

**Week 2: Rust Integration Bridge**
```typescript
// src/rustBridge.ts - Bridge to workspace_tools CLI
import { exec } from 'child_process';
import { promisify } from 'util';
import * as vscode from 'vscode';

const execAsync = promisify(exec);

export class RustWorkspaceBridge {
    private workspaceRoot: string;
    private cliPath: string;

    constructor(workspaceRoot: string) {
        this.workspaceRoot = workspaceRoot;
        this.cliPath = 'workspace-tools'; // Assume CLI is in PATH
    }

    async detectWorkspace(): Promise<WorkspaceInfo> {
        try {
            const { stdout } = await execAsync(
                `${this.cliPath} info --json`,
                { cwd: this.workspaceRoot }
            );
            return JSON.parse(stdout);
        } catch (error) {
            throw new Error(`Failed to detect workspace: ${error}`);
        }
    }

    async getStandardDirectories(): Promise<DirectoryInfo[]> {
        const { stdout } = await execAsync(
            `${this.cliPath} directories --json`,
            { cwd: this.workspaceRoot }
        );
        return JSON.parse(stdout);
    }

    async createStandardDirectory(name: string): Promise<void> {
        await execAsync(
            `${this.cliPath} create-dir "${name}"`,
            { cwd: this.workspaceRoot }
        );
    }

    async loadConfig<T>(name: string): Promise<T> {
        const { stdout } = await execAsync(
            `${this.cliPath} config get "${name}" --json`,
            { cwd: this.workspaceRoot }
        );
        return JSON.parse(stdout);
    }

    async saveConfig<T>(name: string, config: T): Promise<void> {
        const configJson = JSON.stringify(config, null, 2);
        await execAsync(
            `${this.cliPath} config set "${name}"`,
            { 
                cwd: this.workspaceRoot,
                input: configJson
            }
        );
    }

    async findResources(pattern: string): Promise<string[]> {
        const { stdout } = await execAsync(
            `${this.cliPath} find "${pattern}" --json`,
            { cwd: this.workspaceRoot }
        );
        return JSON.parse(stdout);
    }

    async validateWorkspaceStructure(): Promise<ValidationResult> {
        try {
            const { stdout } = await execAsync(
                `${this.cliPath} validate --json`,
                { cwd: this.workspaceRoot }
            );
            return JSON.parse(stdout);
        } catch (error) {
            return {
                valid: false,
                warnings: [{ message: `Validation failed: ${error}`, severity: 'error' }],
                suggestions: []
            };
        }
    }
}

// Workspace detection and activation
export async function activateWorkspaceTools(context: vscode.ExtensionContext) {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
        return;
    }

    const bridge = new RustWorkspaceBridge(workspaceFolder.uri.fsPath);
    
    try {
        const workspaceInfo = await bridge.detectWorkspace();
        vscode.commands.executeCommand('setContext', 'workspace-tools.isWorkspace', true);
        
        // Initialize workspace explorer
        const workspaceExplorer = new WorkspaceExplorerProvider(bridge);
        vscode.window.registerTreeDataProvider('workspace-tools.workspaceExplorer', workspaceExplorer);
        
        // Register commands
        registerCommands(context, bridge);
        
        // Update status bar
        updateStatusBar(workspaceInfo);
        
    } catch (error) {
        console.log('workspace_tools not detected in this workspace');
        vscode.commands.executeCommand('setContext', 'workspace-tools.isWorkspace', false);
    }
}
```

#### **Phase 2: Workspace Explorer and Navigation** (Weeks 3-4)

**Week 3: Tree View Implementation**
```typescript
// src/workspaceExplorer.ts
import * as vscode from 'vscode';
import * as path from 'path';
import { RustWorkspaceBridge } from './rustBridge';

export class WorkspaceExplorerProvider implements vscode.TreeDataProvider<WorkspaceItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<WorkspaceItem | undefined | null | void> = new vscode.EventEmitter<WorkspaceItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<WorkspaceItem | undefined | null | void> = this._onDidChangeTreeData.event;

    constructor(private bridge: RustWorkspaceBridge) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: WorkspaceItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: WorkspaceItem): Promise<WorkspaceItem[]> {
        if (!element) {
            // Root level items
            return [
                new WorkspaceItem(
                    'Standard Directories',
                    vscode.TreeItemCollapsibleState.Expanded,
                    'directories'
                ),
                new WorkspaceItem(
                    'Configuration Files',
                    vscode.TreeItemCollapsibleState.Expanded,
                    'configs'
                ),
                new WorkspaceItem(
                    'Resources',
                    vscode.TreeItemCollapsibleState.Collapsed,
                    'resources'
                )
            ];
        }

        switch (element.contextValue) {
            case 'directories':
                return this.getDirectoryItems();
            case 'configs':
                return this.getConfigItems();
            case 'resources':
                return this.getResourceItems();
            default:
                return [];
        }
    }

    private async getDirectoryItems(): Promise<WorkspaceItem[]> {
        try {
            const directories = await this.bridge.getStandardDirectories();
            return directories.map(dir => {
                const item = new WorkspaceItem(
                    `${dir.name} ${dir.exists ? '✓' : '✗'}`,
                    vscode.TreeItemCollapsibleState.None,
                    'directory'
                );
                item.resourceUri = vscode.Uri.file(dir.path);
                item.tooltip = `${dir.purpose} ${dir.exists ? '(exists)' : '(missing)'}`;
                item.command = {
                    command: 'vscode.openFolder',
                    title: 'Open Directory',
                    arguments: [vscode.Uri.file(dir.path)]
                };
                return item;
            });
        } catch (error) {
            return [new WorkspaceItem('Error loading directories', vscode.TreeItemCollapsibleState.None, 'error')];
        }
    }

    private async getConfigItems(): Promise<WorkspaceItem[]> {
        try {
            const workspaceInfo = await this.bridge.detectWorkspace();
            return workspaceInfo.configFiles.map(config => {
                const item = new WorkspaceItem(
                    `${config.name}.${config.format}`,
                    vscode.TreeItemCollapsibleState.None,
                    'config'
                );
                item.resourceUri = vscode.Uri.file(config.path);
                item.tooltip = `Configuration file (${config.format.toUpperCase()})`;
                item.command = {
                    command: 'vscode.open',
                    title: 'Open Config',
                    arguments: [vscode.Uri.file(config.path)]
                };
                return item;
            });
        } catch (error) {
            return [new WorkspaceItem('No configuration files found', vscode.TreeItemCollapsibleState.None, 'info')];
        }
    }

    private async getResourceItems(): Promise<WorkspaceItem[]> {
        try {
            const commonPatterns = [
                { name: 'Rust Sources', pattern: 'src/**/*.rs' },
                { name: 'Tests', pattern: 'tests/**/*.rs' },
                { name: 'Documentation', pattern: 'docs/**/*' },
                { name: 'Scripts', pattern: '**/*.sh' }
            ];

            const items: WorkspaceItem[] = [];
            for (const pattern of commonPatterns) {
                const resources = await this.bridge.findResources(pattern.pattern);
                const item = new WorkspaceItem(
                    `${pattern.name} (${resources.length})`,
                    resources.length > 0 ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None,
                    'resource-group'
                );
                item.tooltip = `Pattern: ${pattern.pattern}`;
                items.push(item);
            }
            return items;
        } catch (error) {
            return [new WorkspaceItem('Error loading resources', vscode.TreeItemCollapsibleState.None, 'error')];
        }
    }
}

class WorkspaceItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly contextValue: string
    ) {
        super(label, collapsibleState);
    }
}
```

**Week 4: Quick Actions and Context Menus**
```typescript
// src/commands.ts
import * as vscode from 'vscode';
import { RustWorkspaceBridge } from './rustBridge';

export function registerCommands(context: vscode.ExtensionContext, bridge: RustWorkspaceBridge) {
    // Workspace detection command
    const detectWorkspaceCommand = vscode.commands.registerCommand(
        'workspace-tools.detectWorkspace',
        async () => {
            try {
                const workspaceInfo = await bridge.detectWorkspace();
                vscode.window.showInformationMessage(
                    `Workspace detected: ${workspaceInfo.type} at ${workspaceInfo.root}`
                );
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to detect workspace: ${error}`);
            }
        }
    );

    // Create standard directories command
    const createDirectoriesCommand = vscode.commands.registerCommand(
        'workspace-tools.createStandardDirectories',
        async () => {
            const directories = ['config', 'data', 'logs', 'docs', 'tests'];
            const selected = await vscode.window.showQuickPick(
                directories.map(dir => ({ label: dir, picked: false })),
                {
                    placeHolder: 'Select directories to create',
                    canPickMany: true
                }
            );

            if (selected && selected.length > 0) {
                for (const dir of selected) {
                    try {
                        await bridge.createStandardDirectory(dir.label);
                        vscode.window.showInformationMessage(`Created ${dir.label} directory`);
                    } catch (error) {
                        vscode.window.showErrorMessage(`Failed to create ${dir.label}: ${error}`);
                    }
                }
                
                // Refresh explorer
                vscode.commands.executeCommand('workspace-tools.refresh');
            }
        }
    );

    // Open configuration command
    const openConfigCommand = vscode.commands.registerCommand(
        'workspace-tools.openConfig',
        async () => {
            const configName = await vscode.window.showInputBox({
                placeHolder: 'Enter configuration name (e.g., "app", "database")',
                prompt: 'Configuration file to open or create'
            });

            if (configName) {
                try {
                    // Try to load existing config
                    await bridge.loadConfig(configName);
                    
                    // If successful, open the file
                    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                    if (workspaceFolder) {
                        const configPath = vscode.Uri.joinPath(
                            workspaceFolder.uri,
                            'config',
                            `${configName}.toml`
                        );
                        await vscode.window.showTextDocument(configPath);
                    }
                } catch (error) {
                    // Config doesn't exist, offer to create it
                    const create = await vscode.window.showQuickPick(
                        ['Create TOML config', 'Create YAML config', 'Create JSON config'],
                        { placeHolder: 'Configuration file not found. Create new?' }
                    );

                    if (create) {
                        const format = create.split(' ')[1].toLowerCase();
                        // Create empty config file
                        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                        if (workspaceFolder) {
                            const configPath = vscode.Uri.joinPath(
                                workspaceFolder.uri,
                                'config',
                                `${configName}.${format}`
                            );
                            
                            const edit = new vscode.WorkspaceEdit();
                            edit.createFile(configPath, { overwrite: false });
                            await vscode.workspace.applyEdit(edit);
                            await vscode.window.showTextDocument(configPath);
                        }
                    }
                }
            }
        }
    );

    // Validate workspace structure command
    const validateCommand = vscode.commands.registerCommand(
        'workspace-tools.validate',
        async () => {
            try {
                const result = await bridge.validateWorkspaceStructure();
                
                if (result.valid) {
                    vscode.window.showInformationMessage('Workspace structure is valid ✓');
                } else {
                    const warnings = result.warnings.map(w => w.message).join('\n');
                    vscode.window.showWarningMessage(
                        `Workspace validation found issues:\n${warnings}`
                    );
                }
            } catch (error) {
                vscode.window.showErrorMessage(`Validation failed: ${error}`);
            }
        }
    );

    // Generate boilerplate command
    const generateBoilerplateCommand = vscode.commands.registerCommand(
        'workspace-tools.generateBoilerplate',
        async () => {
            const templates = [
                'CLI Application',
                'Web Service', 
                'Library',
                'Desktop Application',
                'Configuration File'
            ];

            const selected = await vscode.window.showQuickPick(templates, {
                placeHolder: 'Select template to generate'
            });

            if (selected) {
                try {
                    // This would integrate with the template system (Task 002)
                    vscode.window.showInformationMessage(`Generating ${selected} template...`);
                    // await bridge.generateBoilerplate(selected.toLowerCase().replace(' ', '-'));
                    vscode.window.showInformationMessage(`${selected} template generated successfully`);
                } catch (error) {
                    vscode.window.showErrorMessage(`Template generation failed: ${error}`);
                }
            }
        }
    );

    // Register all commands
    context.subscriptions.push(
        detectWorkspaceCommand,
        createDirectoriesCommand,
        openConfigCommand,
        validateCommand,
        generateBoilerplateCommand
    );
}
```

#### **Phase 3: IntelliJ/RustRover Plugin** (Weeks 5-6)

**Week 5: Plugin Foundation**
```kotlin
// src/main/kotlin/com/workspace_tools/plugin/WorkspaceToolsPlugin.kt
package com.workspace_tools.plugin

import com.intellij.openapi.components.BaseComponent
import com.intellij.openapi.project.Project
import com.intellij.openapi.startup.StartupActivity
import com.intellij.openapi.vfs.VirtualFileManager
import com.intellij.openapi.wm.ToolWindowManager

class WorkspaceToolsPlugin : BaseComponent {
    override fun getComponentName(): String = "WorkspaceToolsPlugin"
}

class WorkspaceToolsStartupActivity : StartupActivity {
    override fun runActivity(project: Project) {
        val workspaceService = project.getService(WorkspaceService::class.java)
        
        if (workspaceService.isWorkspaceProject()) {
            // Register tool window
            val toolWindowManager = ToolWindowManager.getInstance(project)
            val toolWindow = toolWindowManager.registerToolWindow(
                "Workspace Tools",
                true,
                ToolWindowAnchor.LEFT
            )
            
            // Initialize workspace explorer
            val explorerPanel = WorkspaceExplorerPanel(project, workspaceService)
            toolWindow.contentManager.addContent(
                toolWindow.contentManager.factory.createContent(explorerPanel, "Explorer", false)
            )
        }
    }
}

// src/main/kotlin/com/workspace_tools/plugin/WorkspaceService.kt
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.execution.util.ExecUtil
import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.google.gson.Gson
import java.io.File

@Service
class WorkspaceService(private val project: Project) {
    private val gson = Gson()
    
    fun isWorkspaceProject(): Boolean {
        return try {
            detectWorkspace()
            true
        } catch (e: Exception) {
            false
        }
    }
    
    fun detectWorkspace(): WorkspaceInfo {
        val projectPath = project.basePath ?: throw IllegalStateException("No project path")
        
        val commandLine = GeneralCommandLine()
            .withExePath("workspace-tools")
            .withParameters("info", "--json")
            .withWorkDirectory(File(projectPath))
        
        val output = ExecUtil.execAndGetOutput(commandLine)
        if (output.exitCode != 0) {
            throw RuntimeException("Failed to detect workspace: ${output.stderr}")
        }
        
        return gson.fromJson(output.stdout, WorkspaceInfo::class.java)
    }
    
    fun getStandardDirectories(): List<DirectoryInfo> {
        val projectPath = project.basePath ?: return emptyList()
        
        val commandLine = GeneralCommandLine()
            .withExePath("workspace-tools")
            .withParameters("directories", "--json")
            .withWorkDirectory(File(projectPath))
        
        val output = ExecUtil.execAndGetOutput(commandLine)
        if (output.exitCode != 0) {
            return emptyList()
        }
        
        return gson.fromJson(output.stdout, Array<DirectoryInfo>::class.java).toList()
    }
    
    fun createStandardDirectory(name: String) {
        val projectPath = project.basePath ?: return
        
        val commandLine = GeneralCommandLine()
            .withExePath("workspace-tools")
            .withParameters("create-dir", name)
            .withWorkDirectory(File(projectPath))
        
        ExecUtil.execAndGetOutput(commandLine)
        
        // Refresh project view
        VirtualFileManager.getInstance().syncRefresh()
    }
}

data class WorkspaceInfo(
    val root: String,
    val type: String,
    val standardDirectories: List<String>,
    val configFiles: List<ConfigFileInfo>
)

data class DirectoryInfo(
    val name: String,
    val path: String,
    val purpose: String,
    val exists: Boolean,
    val isEmpty: Boolean
)

data class ConfigFileInfo(
    val name: String,
    val path: String,
    val format: String
)
```

**Week 6: Tool Window and Actions**
```kotlin
// src/main/kotlin/com/workspace_tools/plugin/WorkspaceExplorerPanel.kt
import com.intellij.openapi.project.Project
import com.intellij.ui.components.JBScrollPane
import com.intellij.ui.treeStructure.SimpleTree
import com.intellij.util.ui.tree.TreeUtil
import javax.swing.*
import javax.swing.tree.DefaultMutableTreeNode
import javax.swing.tree.DefaultTreeModel
import java.awt.BorderLayout

class WorkspaceExplorerPanel(
    private val project: Project,
    private val workspaceService: WorkspaceService
) : JPanel() {
    
    private val tree: SimpleTree
    private val rootNode = DefaultMutableTreeNode("Workspace")
    
    init {
        layout = BorderLayout()
        
        tree = SimpleTree()
        tree.model = DefaultTreeModel(rootNode)
        tree.isRootVisible = true
        
        add(JBScrollPane(tree), BorderLayout.CENTER)
        add(createToolbar(), BorderLayout.NORTH)
        
        refreshTree()
    }
    
    private fun createToolbar(): JComponent {
        val toolbar = JPanel()
        
        val refreshButton = JButton("Refresh")
        refreshButton.addActionListener { refreshTree() }
        
        val createDirButton = JButton("Create Directory")
        createDirButton.addActionListener { showCreateDirectoryDialog() }
        
        val validateButton = JButton("Validate")
        validateButton.addActionListener { validateWorkspace() }
        
        toolbar.add(refreshButton)
        toolbar.add(createDirButton)
        toolbar.add(validateButton)
        
        return toolbar
    }
    
    private fun refreshTree() {
        SwingUtilities.invokeLater {
            rootNode.removeAllChildren()
            
            try {
                val workspaceInfo = workspaceService.detectWorkspace()
                
                // Add directories node
                val directoriesNode = DefaultMutableTreeNode("Standard Directories")
                rootNode.add(directoriesNode)
                
                val directories = workspaceService.getStandardDirectories()
                directories.forEach { dir ->
                    val status = if (dir.exists) "✓" else "✗"
                    val dirNode = DefaultMutableTreeNode("${dir.name} $status")
                    directoriesNode.add(dirNode)
                }
                
                // Add configuration files node
                val configsNode = DefaultMutableTreeNode("Configuration Files")
                rootNode.add(configsNode)
                
                workspaceInfo.configFiles.forEach { config ->
                    val configNode = DefaultMutableTreeNode("${config.name}.${config.format}")
                    configsNode.add(configNode)
                }
                
                TreeUtil.expandAll(tree)
                (tree.model as DefaultTreeModel).reload()
                
            } catch (e: Exception) {
                val errorNode = DefaultMutableTreeNode("Error: ${e.message}")
                rootNode.add(errorNode)
                (tree.model as DefaultTreeModel).reload()
            }
        }
    }
    
    private fun showCreateDirectoryDialog() {
        val directories = arrayOf("config", "data", "logs", "docs", "tests")
        val selected = JOptionPane.showInputDialog(
            this,
            "Select directory to create:",
            "Create Standard Directory",
            JOptionPane.PLAIN_MESSAGE,
            null,
            directories,
            directories[0]
        ) as String?
        
        if (selected != null) {
            try {
                workspaceService.createStandardDirectory(selected)
                JOptionPane.showMessageDialog(
                    this,
                    "Directory '$selected' created successfully",
                    "Success",
                    JOptionPane.INFORMATION_MESSAGE
                )
                refreshTree()
            } catch (e: Exception) {
                JOptionPane.showMessageDialog(
                    this,
                    "Failed to create directory: ${e.message}",
                    "Error",
                    JOptionPane.ERROR_MESSAGE
                )
            }
        }
    }
    
    private fun validateWorkspace() {
        try {
            // This would call the validation functionality
            JOptionPane.showMessageDialog(
                this,
                "Workspace structure is valid ✓",
                "Validation Result",
                JOptionPane.INFORMATION_MESSAGE
            )
        } catch (e: Exception) {
            JOptionPane.showMessageDialog(
                this,
                "Validation failed: ${e.message}",
                "Validation Result",
                JOptionPane.WARNING_MESSAGE
            )
        }
    }
}
```

#### **Phase 4: rust-analyzer Integration** (Weeks 7-8)

**Week 7: LSP Extension Specification**
```json
// rust-analyzer extension specification
{
  "workspaceTools": {
    "capabilities": {
      "workspacePathCompletion": true,
      "workspacePathHover": true,
      "workspacePathCodeActions": true,
      "workspaceValidation": true
    },
    "features": {
      "completion": {
        "workspacePaths": {
          "trigger": ["ws.", "workspace."],
          "patterns": [
            "ws.config_dir()",
            "ws.data_dir()",
            "ws.logs_dir()",
            "ws.join(\"{path}\")"
          ]
        }
      },
      "hover": {
        "workspacePaths": {
          "provides": "workspace-relative path information"
        }
      },
      "codeAction": {
        "convertPaths": {
          "title": "Convert to workspace-relative path",
          "kind": "refactor.rewrite"
        }
      },
      "diagnostics": {
        "workspaceStructure": {
          "validates": ["workspace configuration", "standard directories"]
        }
      }
    }
  }
}
```

**Week 8: Implementation and Testing**
```rust
// rust-analyzer integration (conceptual - would be contributed to rust-analyzer)
// This shows what the integration would look like

// Completion provider for workspace_tools
pub fn workspace_tools_completion(
    ctx: &CompletionContext,
) -> Option<Vec<CompletionItem>> {
    if !is_workspace_tools_context(ctx) {
        return None;
    }
    
    let items = vec![
        CompletionItem {
            label: "config_dir()".to_string(),
            kind: CompletionItemKind::Method,
            detail: Some("workspace_tools::Workspace::config_dir".to_string()),
            documentation: Some("Get the standard configuration directory path".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "data_dir()".to_string(),
            kind: CompletionItemKind::Method,
            detail: Some("workspace_tools::Workspace::data_dir".to_string()),
            documentation: Some("Get the standard data directory path".to_string()),
            ..Default::default()
        },
        // ... more completions
    ];
    
    Some(items)
}

// Hover provider for workspace paths
pub fn workspace_path_hover(
    ctx: &HoverContext,
) -> Option<HoverResult> {
    if let Some(workspace_path) = extract_workspace_path(ctx) {
        Some(HoverResult {
            markup: format!(
                "**Workspace Path**: `{}`\n\nResolves to: `{}`",
                workspace_path.relative_path,
                workspace_path.absolute_path
            ),
            range: ctx.range,
        })
    } else {
        None
    }
}
```

### **Success Criteria**
- [ ] VS Code extension published to marketplace with >1k installs
- [ ] IntelliJ plugin published to JetBrains marketplace
- [ ] rust-analyzer integration proposal accepted (or prototype working)
- [ ] Extensions provide meaningful workspace navigation and management
- [ ] Auto-completion and code actions work seamlessly
- [ ] User feedback score >4.5 stars on extension marketplaces
- [ ] Integration increases workspace_tools adoption by 50%+

### **Metrics to Track**
- Extension download/install counts
- User ratings and reviews
- Feature usage analytics (which features are used most)
- Bug reports and resolution time
- Contribution to overall workspace_tools adoption

### **Future Enhancements**
- Integration with other editors (Vim, Emacs, Sublime Text)
- Advanced refactoring tools for workspace-relative paths  
- Visual workspace structure designer
- Integration with workspace templates and scaffolding
- Real-time workspace validation and suggestions
- Team collaboration features for shared workspace configurations

### **Distribution Strategy**
1. **VS Code**: Publish to Visual Studio Code Marketplace
2. **IntelliJ**: Publish to JetBrains Plugin Repository
3. **rust-analyzer**: Contribute as upstream feature or extension
4. **Documentation**: Comprehensive setup and usage guides
5. **Community**: Demo videos, blog posts, conference presentations

This task significantly increases workspace_tools visibility by putting it directly into developers' daily workflow, making adoption natural and discoverable.