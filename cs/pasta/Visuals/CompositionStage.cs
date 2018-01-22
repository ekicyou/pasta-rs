using SharpDX.Direct3D;
using SharpDX.Direct3D11;
using SharpDX.WIC;
using System;
using System.Linq;
using System.Threading;
using System.Collections.Generic;
using SharpDX.Mathematics.Interop;
using D2D = SharpDX.Direct2D1;

using D3D = SharpDX.Direct3D11;

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
        public D2D.RenderTarget D2DTarget { get; private set; }

        public DCOMP.DesktopDevice DevDCOMP { get; private set; }

        public Dictionary<string, D2D.Bitmap> ImageDic { get; private set; }

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

#if false
            // ビットマップはこのあたりの記事を参考にDCVisualに確保したい
            // https://msdn.microsoft.com/ja-jp/magazine/dn781354.aspx
            // https://msdn.microsoft.com/magazine/dn759437

            using (var funcCTS = new CancellationTokenSource().Begin())
            {
                var funcCT = funcCTS.Token;
                // ビットマップの読み込み
                Func<int, int, Texture2D> CreateTexture2D = (width, height) =>
                {
                    var desc = new Texture2DDescription
                    {
                        ArraySize = 1,
                        BindFlags = BindFlags.ShaderResource,
                        CpuAccessFlags = CpuAccessFlags.None,
                        Format = DXGI.Format.B8G8R8A8_UNorm,
                        Height = width,
                        Width = height,
                        MipLevels = 1,
                        OptionFlags = ResourceOptionFlags.None,
                        SampleDescription = new DXGI.SampleDescription(1, 0),
                        Usage = ResourceUsage.Default,
                    };
                    var tx = new Texture2D(DevD3D, desc).RegisterBy(ct);
                    return tx;
                };
                var qBmp = from item in Resources.Const.Shell.AllItems
                           let name = item.Item1
                           let pic_stream = item.Item2
                           let loader = new BitmapLoader(pic_stream, funcCT)
                           let size = loader.Size
                           let tx = CreateTexture2D(size.Width, size.Height)
                           let target = new D2D.RenderTarget()
                           let bmp = loader.CreateBitmap(tx)
            }
#endif
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
            DPI = dbi;

            DisposeWindowResources();
            CreateDeviceResources();
            WindowCTS = new CancellationTokenSource().Begin();
            var ct = WindowCTS.Token;
            Top = DCOMP.Target.FromHwnd(DevDCOMP, hwnd, true).RegisterBy(ct);
            Back = DCOMP.Target.FromHwnd(DevDCOMP, hwnd, false).RegisterBy(ct);

            {
                var root = new DCOMP.Visual2(DevDCOMP).RegisterBy(ct);
                var v1_surface = new DCOMP.Surface(DevDCOMP, Width, Height, DXGI.Format.B8G8R8A8_UNorm, DXGI.AlphaMode.Premultiplied).RegisterBy(ct);
                DrawGeometry(v1_surface);
                var v1 = new DCOMP.Visual2(DevDCOMP).RegisterBy(ct);
                v1.Content = v1_surface;
                root.AddVisual(v1);
                Top.Root = root;
            }
            DevDCOMP.Commit();
        }

        private static void DrawGeometry(DCOMP.Surface comp_surface)
        {
            using (var cts = new CancellationTokenSource().Begin())
            {
                var ct = cts.Token;
                var row_point = default(RawPoint);
                var dc = comp_surface.BeginDraw<D2D.DeviceContext>(null, out row_point).RegisterBy(ct);
                ct.Register(() => comp_surface.EndDraw());
            }
        }
    }
}