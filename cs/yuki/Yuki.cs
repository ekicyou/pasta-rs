using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Setugekka;

namespace Setugekka.Yuki
{
    public class Yuki : IShioriUnsafe
    {
        public IntPtr HInst { get; private set; }
        public string LoadDir { get; private set; }

        /// <summary>SHIORI実体</summary>
        public dynamic Shiori { get; private set; }

        public unsafe int load(void* hinst, void* hGlobal_loaddir, int loaddir_len)
        {
            HInst = new IntPtr(hinst);
            var pload = new IntPtr(hGlobal_loaddir);
            try
            {
                LoadDir = Marshal.PtrToStringAnsi(pload, loaddir_len);
                // SHIORIのロード

                // Load実行
                return Shiori.Load(HInst, LoadDir) == true ? 1 : 0;
            }
            catch (Exception ex)
            {
                Debug.Fail(ex.ToString());
                return 0;
            }
            finally
            {
                Marshal.FreeHGlobal(pload);
            }
        }

        public int unload()
        {
            try
            {
                return Shiori.Unload() == true ? 1 : 0;
            }
            catch (Exception ex)
            {
                Debug.Fail(ex.ToString());
                return 0;
            }
        }

        public unsafe void* request(void* hGlobal_request, int* len)
        {
            var preq = new IntPtr(hGlobal_request);
            try
            {
                // Request実行
                var req = "";
                var res = Shiori.Request(req);

                var pres = IntPtr.Zero;
                *len = 0;
                return pres.ToPointer();
            }
            catch (Exception ex)
            {
                Debug.Fail(ex.ToString());
                *len = 0;
                return IntPtr.Zero.ToPointer();
            }
            finally
            {
                Marshal.FreeHGlobal(preq);
            }
        }
    }
}