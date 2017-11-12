﻿using SharpDX;
using SharpDX.Direct2D1;
using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Windows.Media.Imaging;
using DXGI = SharpDX.DXGI;

namespace Pasta.Visuals
{
    public static   class DxUtils
    {
        /// <summary>
        /// ファイルからビットマップを作成します。
        /// </summary>
        /// <param name="target"></param>
        /// <param name="filename"></param>
        /// <returns></returns>
        public static Bitmap CreateBitmap(this RenderTarget target, string filename)
        {
            var uri = new Uri(filename, UriKind.RelativeOrAbsolute);
            var decoder = BitmapDecoder.Create(uri, BitmapCreateOptions.None, BitmapCacheOption.Default);
            return CreateBitmapImpl(target, decoder);
        }
        /// <summary>
        /// 画像ストリームからビットマップを作成します。
        /// </summary>
        /// <param name="target"></param>
        /// <param name="pic_stream"></param>
        /// <returns></returns>
        public static Bitmap CreateBitmap(this RenderTarget target, Stream pic_stream)
        {
            var decoder = BitmapDecoder.Create(pic_stream, BitmapCreateOptions.None, BitmapCacheOption.Default);
            return CreateBitmapImpl(target, decoder);
        }
        private static Bitmap CreateBitmapImpl(RenderTarget target, BitmapDecoder decoder)
        {
            var source = new FormatConvertedBitmap(decoder.Frames[0], System.Windows.Media.PixelFormats.Pbgra32, null, 0.0);
            var size = new Size2(source.PixelWidth, source.PixelHeight);
            int stride = 4 * size.Width;
            byte[] data = new byte[stride * size.Height];
            source.CopyPixels(data, stride, 0);
            var data_handle = GCHandle.Alloc(data);
            var data_ptr = Marshal.UnsafeAddrOfPinnedArrayElement(data, 0);
            var stream = new DataStream(data_ptr, data.LongLength, true, false);
            var properties = new BitmapProperties
            {
                PixelFormat = new PixelFormat(DXGI.Format.B8G8R8A8_UNorm, AlphaMode.Premultiplied),
            };
            return new Bitmap(target, size, stream, stride, properties);
        }
    }
}
