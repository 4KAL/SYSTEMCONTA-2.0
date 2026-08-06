; Sistema Contabilidad - Instalador Inno Setup
; Compilar con: iscc SistemaContabilidad_installer.iss  (requiere Inno Setup + internet)
; La app guarda su DB en <carpeta_exes>/../contabilidad_rust.db; si no existe,
; la crea en el primer inicio migrando el legacy de %APPDATA%\SistemaConta\contabilidad.db.

#define AppName "Sistema Contabilidad"
#define AppVer  "1.0.0"

[Setup]
AppName={#AppName}
AppVersion={#AppVer}
AppPublisher=Sistema Contabilidad
DefaultDirName={localappdata}\SistemaContabilidad
DefaultGroupName={#AppName}
OutputBaseFilename=SistemaContabilidad-{#AppVer}-setup
Compression=lzma
SolidCompression=yes
PrivilegesLimit=lowest
ArchitecturesAllowed=x64
ArchitecturesAllowed=x86
ArchitecturesPrefer=x64

[Files]
Source: "release\sistema-contabilidad.exe"; DestDir: {app}; DestName: "SistemaContabilidad.exe"; Flags: ignoreversion
Source: "release\server.exe"; DestDir: {app}; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\SistemaContabilidad.exe"
Name: "{commondesktop}\{#AppName}"; Filename: "{app}\SistemaContabilidad.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Crear acceso directo en el escritorio"; GroupDescription:; Flags: unchecked

[Run]
Filename: "{app}\SistemaContabilidad.exe"; Description: "Abrir {#AppName}"; Flags: nowait postinstall skipifsdoesntexist
