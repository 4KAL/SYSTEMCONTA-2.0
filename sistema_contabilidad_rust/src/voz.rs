use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const VOZ_SCRIPT: &str = r#"param([string]$Texto = $env:VOZ_TEXTO)
$ErrorActionPreference = 'SilentlyContinue'

# Intento 1: Edge TTS (voz neural Dalia por internet, sin API de pago)
function Usar-EdgeTTS {
    try {
        $py = $null
        foreach ($candidato in @('py', 'python', 'python3')) {
            $cmd = Get-Command $candidato -ErrorAction SilentlyContinue
            if ($cmd) { $py = $cmd.Source; break }
        }
        if (-not $py) {
            $lp = $env:LOCALAPPDATA
            foreach ($v in @('Python314', 'Python313', 'Python312', 'Python311', 'Python310')) {
                $p = Join-Path $lp ("Programs\Python\$v\python.exe")
                if (Test-Path $p) { $py = $p; break }
            }
        }
        if (-not $py) { return $false }
        $script = Join-Path $PSScriptRoot 'tts_edge.py'
        if (-not (Test-Path $script)) { return $false }
        $ruta = Join-Path $env:TEMP ("voz_" + [System.Guid]::NewGuid().ToString() + ".mp3")
        $textoArg = $Texto.Replace('"', "'")
        $argsStr = '"{0}" "{1}" "{2}"' -f $script, $textoArg, $ruta
        $proc = Start-Process -FilePath $py -ArgumentList $argsStr -WindowStyle Hidden -PassThru
        $limite = (Get-Date).AddSeconds(60)
        while (-not $proc.HasExited -and (Get-Date) -lt $limite) { Start-Sleep -Milliseconds 200 }
        if (-not $proc.HasExited) { $proc.Kill(); return $false }
        if ($proc.ExitCode -ne 0) { return $false }
        if (-not (Test-Path $ruta)) { return $false }
        if ((Get-Item $ruta).Length -le 0) { return $false }
        Add-Type -TypeDefinition @'
using System.Runtime.InteropServices;
using System.Text;
public class Mci {
    [DllImport("winmm.dll")]
    public static extern int mciSendString(string command, StringBuilder ret, int retlen, System.IntPtr hwnd);
}
'@
        $alias = 'v' + [System.Guid]::NewGuid().ToString("N").Substring(0, 8)
        $rcOpen = [Mci]::mciSendString('open "' + $ruta + '" type mpegvideo alias ' + $alias, $null, 0, [IntPtr]::Zero)
        if ($rcOpen -ne 0) { return $false }
        [Mci]::mciSendString('play ' + $alias + ' wait', $null, 0, [IntPtr]::Zero) | Out-Null
        [Mci]::mciSendString('close ' + $alias, $null, 0, [IntPtr]::Zero) | Out-Null
        Remove-Item $ruta -ErrorAction SilentlyContinue
        return $true
    } catch {
        return $false
    }
}

if (Usar-EdgeTTS) { exit 0 }

# Intento 2: WinRT (usa voces neurales "Natural" si estan instaladas)
function Usar-WinRT {
    try {
        Add-Type -AssemblyName System.Runtime.WindowsRuntime
        $null = [Windows.Media.SpeechSynthesis.SpeechSynthesizer,Windows.Media.SpeechSynthesis,ContentType=WindowsRuntime]
        $asTaskGeneric = ([System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object {
            $_.Name -eq 'AsTask' -and $_.GetParameters().Count -eq 1 -and
            $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1'
        })[0]
        function Await($WinRtTask, $ResultType) {
            $asTask = $asTaskGeneric.MakeGenericMethod($ResultType)
            $netTask = $asTask.Invoke($null, @($WinRtTask))
            $netTask.Wait(-1) | Out-Null
            $netTask.Result
        }
        $synth = New-Object Windows.Media.SpeechSynthesis.SpeechSynthesizer
        $voices = [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::AllVoices
        $elegida = $null
        foreach ($v in $voices) {
            if ($v.Language -like 'es-*' -and $v.Gender.ToString() -eq 'Female' -and $v.Description -like '*Natural*') { $elegida = $v; break }
        }
        if (-not $elegida) {
            foreach ($v in $voices) {
                if ($v.Language -like 'es-*' -and $v.Gender.ToString() -eq 'Female' -and $v.Description -like '*Online*') { $elegida = $v; break }
            }
        }
        if (-not $elegida) {
            foreach ($v in $voices) {
                if ($v.Language -like 'es-*' -and $v.Gender.ToString() -eq 'Female') { $elegida = $v; break }
            }
        }
        if (-not $elegida) {
            foreach ($v in $voices) {
                if ($v.Language -like 'es-*') { $elegida = $v; break }
            }
        }
        if (-not $elegida) {
            foreach ($v in $voices) {
                if ($v.Gender.ToString() -eq 'Female') { $elegida = $v; break }
            }
        }
        if (-not $elegida) { return $false }
        $synth.Voice = $elegida
        try {
            $synth.Options.SpeakingRate = 0.75
            $synth.Options.Pitch = 0.9
            $synth.Options.AppendedSilence = 0
        } catch { }
        $stream = Await ($synth.SynthesizeTextToStreamAsync($Texto)) ([Windows.Media.SpeechSynthesis.SpeechSynthesisStream])
        $input = [System.IO.WindowsRuntimeStreamExtensions]::AsStreamForRead($stream)
        $ruta = Join-Path $env:TEMP ("voz_" + [System.Guid]::NewGuid().ToString() + ".wma")
        $file = [System.IO.File]::Create($ruta)
        try { $input.CopyTo($file) } finally { $file.Close() }
        Add-Type -AssemblyName PresentationCore
        $player = New-Object System.Windows.Media.MediaPlayer
        $player.Open([Uri]::new($ruta))
        $inicio = Get-Date
        while ((Get-Date) -lt $inicio.AddSeconds(120)) {
            Start-Sleep -Milliseconds 150
            if ($player.HasAudio -and $player.NaturalDuration.HasTimeSpan) {
                if ($player.Position -gt [TimeSpan]::Zero -and $player.Position -ge $player.NaturalDuration.TimeSpan) { break }
            }
        }
        $player.Close()
        $synth.Dispose()
        Remove-Item $ruta -ErrorAction SilentlyContinue
        return $true
    } catch {
        return $false
    }
}

if (Usar-WinRT) { exit 0 }

# Intento 3: SAPI 5 clasico (respaldo)
Add-Type -AssemblyName System.Speech
$s = New-Object System.Speech.Synthesis.SpeechSynthesizer
try {
    $voz = $null
    foreach ($v in $s.GetInstalledVoices()) {
        if (-not $v.Enabled) { continue }
        $info = $v.VoiceInfo
        if ($info.Culture.Name -like 'es-*' -and $info.Gender.ToString() -eq 'Female') { $voz = $info.Name; break }
    }
    if (-not $voz) {
        foreach ($v in $s.GetInstalledVoices()) {
            if (-not $v.Enabled) { continue }
            if ($v.VoiceInfo.Culture.Name -like 'es-*') { $voz = $v.VoiceInfo.Name; break }
        }
    }
    if (-not $voz) {
        foreach ($v in $s.GetInstalledVoices()) {
            if (-not $v.Enabled) { continue }
            if ($v.VoiceInfo.Gender.ToString() -eq 'Female') { $voz = $v.VoiceInfo.Name; break }
        }
    }
    if ($voz) { $s.SelectVoice($voz) }
    $s.Rate = -1
    $s.Volume = 100
    $s.Speak($Texto)
} finally {
    $s.Dispose()
}
"#;

const TTS_EDGE_PY: &str = r#"import asyncio, sys, pathlib

async def main():
    texto = sys.argv[1] if len(sys.argv) > 1 else "Hola"
    salida = sys.argv[2] if len(sys.argv) > 2 else str(pathlib.Path.home() / "voz_temp.mp3")
    try:
        import edge_tts
        comunicador = edge_tts.Communicate(texto, "es-MX-DaliaNeural", rate="+0%")
        with open(salida, "wb") as f:
            async for chunk in comunicador.stream():
                if chunk["type"] == "audio":
                    f.write(chunk["data"])
        print("OK")
    except Exception:
        sys.exit(1)

asyncio.run(main())
"#;

const INSTALAR_TTS_SCRIPT: &str = r#"$ErrorActionPreference = 'SilentlyContinue'
$exeDir = $PSScriptRoot
$marcadorOk = Join-Path $exeDir 'tts_ok.txt'
$marcadorFail = Join-Path $exeDir 'tts_fail.txt'

function Encontrar-Python {
    foreach ($candidato in @('py', 'python', 'python3')) {
        $cmd = Get-Command $candidato -ErrorAction SilentlyContinue
        if ($cmd) { return $cmd.Source }
    }
    $lp = $env:LOCALAPPDATA
    foreach ($v in @('Python314', 'Python313', 'Python312', 'Python311', 'Python310')) {
        $p = Join-Path $lp ("Programs\Python\$v\python.exe")
        if (Test-Path $p) { return $p }
    }
    return $null
}

function Tiene-EdgeTts([string]$py) {
    & $py -c "import edge_tts" 2>$null | Out-Null
    return ($LASTEXITCODE -eq 0)
}

$py = Encontrar-Python
if ($py -and (Tiene-EdgeTts $py)) {
    Set-Content -Path $marcadorOk -Value "ok"
    Remove-Item $marcadorFail -ErrorAction SilentlyContinue
    exit 0
}

$instalador = Join-Path $env:TEMP 'python-3.13.14-amd64.exe'
$url = 'https://www.python.org/ftp/python/3.13.14/python-3.13.14-amd64.exe'
if (-not (Test-Path $instalador)) {
    try {
        Invoke-WebRequest -Uri $url -OutFile $instalador -UseBasicParsing
    } catch {
        Set-Content -Path $marcadorFail -Value "descarga: $($_.Exception.Message)"
        exit 1
    }
}

$proc = Start-Process -FilePath $instalador -ArgumentList '/quiet InstallAllUsers=0 PrependPath=1 Include_launcher=1 Include_pip=1 Include_test=0 Include_doc=0 Include_tcltk=0 Include_tools=0' -Wait -PassThru -WindowStyle Hidden
if ($proc.ExitCode -ne 0) {
    Set-Content -Path $marcadorFail -Value "instalador: exit $($proc.ExitCode)"
    exit 1
}

$py = Encontrar-Python
if (-not $py) {
    Set-Content -Path $marcadorFail -Value "python no encontrado tras instalar"
    exit 1
}

& $py -m pip install --disable-pip-version-check --quiet edge-tts 2>$null
if (Tiene-EdgeTts $py) {
    Set-Content -Path $marcadorOk -Value "ok"
    Remove-Item $marcadorFail -ErrorAction SilentlyContinue
    exit 0
}

Set-Content -Path $marcadorFail -Value "pip edge-tts fallo"
exit 1
"#;

fn ruta_script() -> Option<PathBuf> {
    let dir = std::env::current_exe()
        .ok()?
        .parent()
        .map(|p| p.to_path_buf())?;
    let ruta = dir.join("voz.ps1");
    let _ = std::fs::write(&ruta, VOZ_SCRIPT);
    let _ = std::fs::write(dir.join("tts_edge.py"), TTS_EDGE_PY);
    let _ = std::fs::write(dir.join("instalar_tts.ps1"), INSTALAR_TTS_SCRIPT);
    Some(ruta)
}

pub fn hablar(texto: &str) {
    #[cfg(windows)]
    {
        if let Some(ruta) = ruta_script() {
            let _ = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-WindowStyle",
                    "Hidden",
                    "-File",
                ])
                .arg(&ruta)
                .env("VOZ_TEXTO", texto)
                .creation_flags(0x08000000)
                .spawn();
        }
    }
}

pub fn asegurar_motor() {
    #[cfg(windows)]
    {
        if let Some(dir) = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        {
            let _ = ruta_script();
            let ok = dir.join("tts_ok.txt");
            let fail = dir.join("tts_fail.txt");
            if ok.exists() {
                return;
            }
            if fail.exists() {
                if let Ok(md) = std::fs::metadata(&fail) {
                    if let Ok(t) = md.modified() {
                        let antiguedad = std::time::SystemTime::now()
                            .duration_since(t)
                            .unwrap_or_default();
                        if antiguedad < std::time::Duration::from_secs(24 * 3600) {
                            return;
                        }
                    }
                }
            }
            let _ = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-WindowStyle",
                    "Hidden",
                    "-File",
                ])
                .arg(dir.join("instalar_tts.ps1"))
                .creation_flags(0x08000000)
                .spawn();
        }
    }
}
