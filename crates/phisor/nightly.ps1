# phisor's nightly learning (docs/phisor/learn.md). Takes the next sources
# from data/learn/source.tsv, resumes from the newest saved night, and saves
# tonight's trees, its row in night.tsv and the page data/learn/learned.html.
# Runs at low priority in its own target folder, so a build in the day never
# waits on it. Task Scheduler runs it every night ("phisor nightly").
param([int]$Take = 2)

$phinet = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$logs = Join-Path $PSScriptRoot 'data\learn\log'
New-Item -ItemType Directory -Force $logs | Out-Null
$stamp = Get-Date -Format 'yyyy-MM-dd-HHmm'
$log = Join-Path $logs "$stamp.txt"
Set-Location $phinet
$run = "cargo run --release -p phisor --example nightly --target-dir target/nightly -- --take $Take --stamp $stamp"
cmd /c "start /b /low /wait $run > `"$log`" 2>&1"
"exit $LASTEXITCODE" | Out-File -Append -Encoding utf8 $log
