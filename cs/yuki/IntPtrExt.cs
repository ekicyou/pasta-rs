using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.InteropServices;

namespace Setugekka.Yuki
{
    public static class IntPtrExt
    {
        private static Encoding Enc => Encoding.UTF8;

        /// <summary>
        /// ポインタ情報をUTF8と見なして文字列に変換します。
        /// </summary>
        /// <returns></returns>
        public static string ToUtf8String(this IntPtr p, int byte_count)
        {
            unsafe
            {
                return Enc.GetString((byte*)p.ToPointer(), byte_count);
            }
        }

        /// <summary>
        /// 文字列をUTF8と見なしてバイト列に変換し、HGlobal領域に確保します。
        /// </summary>
        /// <param name="text"></param>
        /// <returns></returns>
        public static Tuple<IntPtr, int> ToHGlobal(this string text)
        {
            var bytes = Enc.GetBytes(text);
            var length = bytes.Length;
            var hg = Marshal.AllocHGlobal(bytes.Length);
            Marshal.Copy(bytes, 0, hg, bytes.Length);
            return Tuple.Create(hg, length);
        }
    }
}