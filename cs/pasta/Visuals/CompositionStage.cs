using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Threading;
using SharpDX.DirectComposition;
using SharpDX.Direct3D11;
using SharpDX.DXGI;
using SharpDX.Direct3D;
using System.Windows.Interop;

using D3D = SharpDX.Direct3D11;

using D2D = SharpDX.Direct2D1;

using DCOMP = SharpDX.DirectComposition;

using DXGI = SharpDX.DXGI;

namespace Pasta.Visuals
{
    public class CompositionStage : IDisposable
    {
        private static NLog.Logger Log { get; } = NLog.LogManager.GetCurrentClassLogger();

        public void Dispose()
        {
            DisposeDeviceResources();
        }

        public ICTSCancelDisposable DeviceCTS { get; private set; }
        public D3D.Device DevD3D { get; private set; }
        public DXGI.Device DevDXGI { get; private set; }
        public D2D.Device DevD2D { get; private set; }
        public DCOMP.DesktopDevice DevDCOMP { get; private set; }
        public bool IsDeviceCreated => DeviceCTS != null;

        public void DisposeDeviceResources()
        {
            var cts = DeviceCTS;
            DeviceCTS = null;
            cts?.Dispose();
        }

        /// <summary>
        /// D3Dデバイスを作成し、デバイスリソースを確保します。
        /// </summary>
        public void CreateDeviceResources()
        {
            if (IsDeviceCreated) return;
            DeviceCTS = new CancellationTokenSource().Begin();
            var ct = DeviceCTS.Token;
            DevD3D = new D3D.Device(DriverType.Hardware, DeviceCreationFlags.BgraSupport).RegisterBy(ct);
            DevDXGI = DevD3D.QueryInterface<DXGI.Device>();
            DevD2D = new D2D.Device(DevDXGI).RegisterBy(ct);
            DevDCOMP = new DCOMP.DesktopDevice(DevDXGI).RegisterBy(ct);

            // 最後にWindowリソースの開放を登録する。
            ct.Register(DisposeWindowResources);
        }

        public ICTSCancelDisposable WindowCTS { get; private set; }
        public IntPtr HWND { get; private set; }
        public int Width { get; private set; }
        public int Height { get; private set; }
        public double DPI { get; private set; }
        public DCOMP.Target Top { get; private set; }
        public DCOMP.Target Back { get; private set; }

        public bool IsWindowCreated => WindowCTS != null;

        public void DisposeWindowResources()
        {
            var cts = WindowCTS;
            WindowCTS = null;
            cts?.Dispose();
        }

        /// <summary>
        /// 指定したウィンドウ情報で領域を作り直します。
        /// </summary>
        /// <param name="hwnd">ウィンドウハンドル</param>
        /// <param name="width">クライアント領域の幅</param>
        /// <param name="height">クライアント領域の高さ</param>
        /// <param name="dbi">クライアント領域のDPI</param>
        public void CreateWindowResources(IntPtr hwnd, int width, int height, double dbi)
        {
            Log.Trace("CreateWindowResources");
            HWND = hwnd;
            Width = width;
            Height = height;
            DPI = DPI;

            DisposeWindowResources();
            CreateDeviceResources();
            WindowCTS = new CancellationTokenSource().Begin();
            var ct = WindowCTS.Token;
            Top = DCOMP.Target.FromHwnd(DevDCOMP, hwnd, true).RegisterBy(ct);
            Back = DCOMP.Target.FromHwnd(DevDCOMP, hwnd, false).RegisterBy(ct);
        }
    }
}