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
        /// <summary>shiori.dllのHInst。</summary>
        public IntPtr HInst { get; private set; }

        /// <summary>shiori.dllが存在するディレクトリ。load apiで渡される。</summary>
        public string LoadDir { get; private set; }

        /// <summary>Hana(SHIORI)実体</summary>
        public dynamic Hana { get; private set; }

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

                // Hana のロード
                Hana = AppDomain.CurrentDomain.CreateInstanceAndUnwrap(assembly_name, type_name);

                // Load実行
                bool rc = Hana.Load(HInst, LoadDir);
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
                bool rc = Hana.Unload();
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
                string res = Hana.Request(req);
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