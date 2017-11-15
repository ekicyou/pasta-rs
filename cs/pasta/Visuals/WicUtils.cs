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

namespace Pasta.Visuals
{
    public static class WicUtils
    {
        /// <summary>
        /// 画像ストリームからD2Dビットマップを作成します。
        /// </summary>
        /// <param name="target"></param>
        /// <param name="pic_stream"></param>
        /// <returns></returns>
        public static D2D.Bitmap CreateBitmap2(D2D.RenderTarget target, Stream pic_stream)
        {
            using (var dec = new BitmapDecoder(DxFactry.WIC, pic_stream, DecodeOptions.CacheOnLoad))
            using (var frame = dec.GetFrame(0))
            using (var cnv = new FormatConverter(DxFactry.WIC))
            {
                cnv.Initialize(frame, PixelFormat.Format32bppPBGRA, BitmapDitherType.None, null, 0, BitmapPaletteType.Custom);
                var bmp = D2D.Bitmap.FromWicBitmap(target, cnv);
                return bmp;
            }
        }
    }
}