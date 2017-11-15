using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using DXGI = SharpDX.DXGI;
using SharpDX.WIC;
using SharpDX;
using System.IO;
using D2D = SharpDX.Direct2D1;
using System.Threading;

namespace Pasta.Visuals
{
    public class BitmapLoader
    {
        public BitmapDecoder Decoder { get; private set; }
        public BitmapFrameDecode Frame { get; private set; }

        public Size2 Size => Frame.Size;

        public BitmapLoader(Stream pic_stream, CancellationToken ct)
        {
            Decoder = new BitmapDecoder(DxFactry.WIC, pic_stream, DecodeOptions.CacheOnLoad).RegisterBy(ct);
            Frame = Decoder.GetFrame(0).RegisterBy(ct);
        }

        public BitmapLoader(string path, CancellationToken ct) : this(File.OpenRead(path), ct)
        {
        }

        public D2D.Bitmap CreateBitmap(D2D.RenderTarget target)
        {
            using (var cnv = new FormatConverter(DxFactry.WIC))
            {
                cnv.Initialize(Frame, PixelFormat.Format32bppPBGRA, BitmapDitherType.None, null, 0, BitmapPaletteType.Custom);
                var bmp = D2D.Bitmap.FromWicBitmap(target, cnv);
                return bmp;
            }
        }
    }
}