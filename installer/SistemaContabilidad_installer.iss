; Sistema Contabilidad - Instalador Inno Setup
; Compilar con: iscc SistemaContabilidad_installer.iss  (requiere Inno Setup 6)
; Instalacion per-usuario (sin elevacion). La app guarda su DB en <carpeta_exes>/../contabilidad_rust.db;
; si no existe, la crea en el primer arranque migrando el legacy de %APPDATA%\SistemaConta\contabilidad.db.

#define AppName "Sistema Contabilidad"
#define AppVer  "1.0.0"

[Setup]
AppName={#AppName}
AppVersion={#AppVer}
AppPublisher=Sistema Contabilidad
DefaultDirName={localappdata}\SistemaContabilidad
DefaultGroupName={#AppName}
PrivilegesRequired=lowest
OutputBaseFilename=SistemaContabilidad-{#AppVer}-setup
OutputDir=Output
Compression=lzma
SolidCompression=yes
ArchitecturesAllowed=x64os

[Files]
Source: "release\sistema-contabilidad.exe"; DestDir: {app}; DestName: "SistemaContabilidad.exe"; Flags: ignoreversion
Source: "release\server.exe"; DestDir: {app}; Flags: ignoreversion

[Icons]
Name: "{autostartmenu}\{#AppName}"; Filename: "{app}\SistemaContabilidad.exe"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\SistemaContabilidad.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Crear acceso directo en el escritorio"; Flags: unchecked

[Run]
Filename: "{app}\SistemaContabilidad.exe"; Description: "Abrir {#AppName}"; Flags: nowait postinstall skipifdoesntexist
