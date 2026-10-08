const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

let lspProcess = null;
let statusBarItem = null;

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    const config = vscode.workspace.getConfiguration('widya');
    const widyaBinary = config.get('executablePath') || 'widya';

    // 1. Status Bar Item
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = "$(gear) Widya-Lang";
    statusBarItem.tooltip = "Widya-Lang Developer Tooling Active (LSP & DAP Ready)";
    statusBarItem.command = 'widya.showInfo';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);

    // 2. Command: Tampilkan Info Lingkungan Widya
    const cmdInfo = vscode.commands.registerCommand('widya.showInfo', () => {
        cp.exec(`${widyaBinary} --version`, (err, stdout, stderr) => {
            if (err) {
                vscode.window.showWarningMessage(
                    `Widya-Lang binary '${widyaBinary}' tidak ditemukan dalam PATH. Pastikan compiler widya sudah terpasang.`,
                    'Pengaturan'
                ).then(selection => {
                    if (selection === 'Pengaturan') {
                        vscode.commands.executeCommand('workbench.action.openSettings', 'widya.executablePath');
                    }
                });
            } else {
                vscode.window.showInformationMessage(`Widya-Lang Aktif: ${stdout.trim()}`);
            }
        });
    });
    context.subscriptions.push(cmdInfo);

    // 3. Command: Jalankan Berkas Widya Aktif
    const cmdRun = vscode.commands.registerCommand('widya.runFile', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || !editor.document.fileName.endsWith('.wya') && !editor.document.fileName.endsWith('.widya')) {
            vscode.window.showWarningMessage("Buka berkas sumber Widya (.wya / .widya) untuk dijalankan.");
            return;
        }
        const filePath = editor.document.fileName;
        const terminal = vscode.window.createTerminal('Widya Run');
        terminal.show();
        terminal.sendText(`${widyaBinary} jalankan "${filePath}"`);
    });
    context.subscriptions.push(cmdRun);

    // 4. Command: Jalankan Pengujian Unit (Test Runner)
    const cmdTest = vscode.commands.registerCommand('widya.testFile', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage("Buka berkas uji Widya untuk menjalankan tes.");
            return;
        }
        const filePath = editor.document.fileName;
        const terminal = vscode.window.createTerminal('Widya Test');
        terminal.show();
        terminal.sendText(`${widyaBinary} uji "${filePath}"`);
    });
    context.subscriptions.push(cmdTest);

    // 5. Command: Format Dokumen
    const cmdFormat = vscode.commands.registerCommand('widya.formatFile', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;
        const filePath = editor.document.fileName;
        cp.exec(`${widyaBinary} format -t "${filePath}"`, (err, stdout, stderr) => {
            if (err) {
                vscode.window.showErrorMessage(`Gagal memformat kode: ${stderr || err.message}`);
            } else {
                vscode.window.showInformationMessage("Berkas Widya berhasil diformat.");
            }
        });
    });
    context.subscriptions.push(cmdFormat);

    // 6. Command: Pasok / Vendor Dependensi Proyek (WPM Offline Mode)
    const cmdVendor = vscode.commands.registerCommand('widya.vendorPackages', () => {
        const wsFolders = vscode.workspace.workspaceFolders;
        if (!wsFolders || wsFolders.length === 0) {
            vscode.window.showWarningMessage("Buka folder proyek Widya untuk memasok dependensi.");
            return;
        }
        const rootPath = wsFolders[0].uri.fsPath;
        const terminal = vscode.window.createTerminal('Widya WPM');
        terminal.show();
        terminal.sendText(`cd "${rootPath}" && ${widyaBinary} pasok`);
        vscode.window.showInformationMessage("Memulai proses WPM Vendor (Air-Gapped Offline Caching)...");
    });
    context.subscriptions.push(cmdVendor);

    // 7. Debug Configuration Provider
    const debugProvider = {
        resolveDebugConfiguration(folder, config, token) {
            if (!config.type && !config.request && !config.name) {
                const editor = vscode.window.activeTextEditor;
                if (editor && (editor.document.languageId === 'widya' || editor.document.fileName.endsWith('.wya'))) {
                    config.type = 'widya';
                    config.name = 'Widya Launch';
                    config.request = 'launch';
                    config.program = '${file}';
                    config.stopOnEntry = false;
                }
            }
            if (!config.program) {
                vscode.window.showErrorMessage("Tidak dapat meluncurkan debugger: program belum ditentukan.");
                return null;
            }
            return config;
        }
    };
    context.subscriptions.push(vscode.debug.registerDebugConfigurationProvider('widya', debugProvider));
}

function deactivate() {
    if (lspProcess) {
        lspProcess.kill();
        lspProcess = null;
    }
    if (statusBarItem) {
        statusBarItem.dispose();
    }
}

module.exports = {
    activate,
    deactivate
};
