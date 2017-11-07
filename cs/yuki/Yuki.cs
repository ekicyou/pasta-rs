using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Setugekka;
using System.IO;

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

                // despript.txtの確認
                var despript_path = Path.Combine(LoadDir, "despript.txt");
                var despript = DescriptExt.Read(despript_path);
                var assembly_name = despript.GetOrDefault("setugekka.hana.assembly", "hana");
                var type_name = despript.GetOrDefault("setugekka.hana.type", "Setugekka.Hana.Hana");

                // SHIORIのロード
                Shiori = AppDomain.CurrentDomain.CreateInstanceAndUnwrap(assembly_name, type_name);

                // Load実行
                bool rc = Shiori.Load(HInst, LoadDir);
                return rc == true ? 1 : 0;
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
                bool rc = Shiori.Unload();
                return rc == true ? 1 : 0;
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
                var req = preq.ToUtf8String(*len);
                String res = Shiori.Request(req);
                var t = res.ToHGlobal();
                var pres = t.Item1;
                *len = t.Item2;
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