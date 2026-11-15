$ErrorActionPreference = "Stop"
$Root = (Resolve-Path "$PSScriptRoot\..").Path
$Assets = Join-Path $Root "assets"

Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class Shot {
	[StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
	[DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
	[DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr h);
	[DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
	[DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int cx, int cy, uint flags);
	[DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
	[DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr dc, uint flags);
	[DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
	[DllImport("user32.dll")] public static extern void mouse_event(uint f, uint dx, uint dy, uint d, UIntPtr e);
}
'@
[void][Shot]::SetProcessDPIAware()

$Window = (Get-Process dawpresence | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1).MainWindowHandle
if (-not $Window) { throw "dawpresence window is not open" }

[void][Shot]::ShowWindow($Window, 9)
[void][Shot]::SetWindowPos($Window, [IntPtr](-1), 0, 0, 0, 0, 0x3)
[void][Shot]::SetWindowPos($Window, [IntPtr](-2), 0, 0, 0, 0, 0x3)
Start-Sleep -Milliseconds 300

$Scale = [Shot]::GetDpiForWindow($Window) / 96.0
$Rect = New-Object Shot+RECT
[void][Shot]::GetWindowRect($Window, [ref]$Rect)

foreach ($Page in @{ Name = "home"; Y = 54 }, @{ Name = "settings"; Y = 98 }) {
	[void][Shot]::SetCursorPos($Rect.L + [int](40 * $Scale), $Rect.T + [int]($Page.Y * $Scale))
	Start-Sleep -Milliseconds 200

	[Shot]::mouse_event(0x2, 0, 0, 0, [UIntPtr]::Zero)
	[Shot]::mouse_event(0x4, 0, 0, 0, [UIntPtr]::Zero)
	Start-Sleep -Milliseconds 500

	$Bitmap = New-Object System.Drawing.Bitmap ($Rect.R - $Rect.L), ($Rect.B - $Rect.T)
	$Graphics = [System.Drawing.Graphics]::FromImage($Bitmap)

	[void][Shot]::PrintWindow($Window, $Graphics.GetHdc(), 2)
	$Graphics.ReleaseHdc()
	$Graphics.Dispose()

	$Bitmap.Save((Join-Path $Assets "$($Page.Name).png"), [System.Drawing.Imaging.ImageFormat]::Png)
	$Bitmap.Dispose()
}
