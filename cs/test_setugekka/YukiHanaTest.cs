using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace test_setugekka
{
    using System.Runtime.InteropServices;
    using System.Threading;
    using Setugekka.Tuki;
    using Setugekka.Yuki;

    [TestClass]
    public class YukiHanaTest
    {
        [TestMethod]
        public void load_yuki_hana()
        {
            var cts = new CancellationTokenSource();
            try
            {
                var ct = cts.Token;

                // create
                var yuki = new Yuki();

                // load
                unsafe
                {
                    var hinst = IntPtr.Zero;
                    var load_dir = AssemblyUtil.GetCallingAssemblyDirctory();
                    var load_t = load_dir.ToHGlobal();
                    var load_hg = load_t.Item1;
                    var load_len = load_t.Item2;
                    var rc = yuki.load(hinst.ToPointer(), load_hg.ToPointer(), load_len);
                    rc.Is(1);
                }
                // unload
                {
                    var rc = yuki.unload();
                    rc.Is(1);
                }
            }
            finally
            {
                cts.Cancel();
                cts.Dispose();
            }
        }
    }
}