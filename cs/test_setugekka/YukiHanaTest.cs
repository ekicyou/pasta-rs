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
        private static NLog.ILogger Log { get; } = NLog.LogManager.GetCurrentClassLogger();

        [TestMethod]
        public void load_yuki_hana()
        {
            var cts = new CancellationTokenSource();
            try
            {
                var ct = cts.Token;
                ct.Register(() =>
                {
                    GC.Collect();
                    GC.WaitForPendingFinalizers();
                });
                ct.Register(() => Log.Trace("finally"));

                // create
                Log.Trace("create");
                var yuki = new Yuki();

                // load
                unsafe
                {
                    Log.Trace("load");
                    var hinst = IntPtr.Zero;
                    var load_dir = AssemblyUtil.GetCallingAssemblyDirctory();
                    var load_t = load_dir.ToHGlobal();
                    var load_h = load_t.Item1;
                    var load_l = load_t.Item2;
                    var rc = yuki.load(hinst.ToPointer(), load_h.ToPointer(), load_l);
                    rc.Is(1);
                }
                // request
                unsafe
                {
                    Log.Trace("request");
                    var req_lines = new[]
                    {
                        "GET SHIORI/3.0",
                        "Charset: UTF-8",
                        "ID: version",
                        "SecurityLevel: local",
                        "Sender: SSP",
                        "",
                    };
                    var reg_str = string.Join("\r\n", req_lines);
                    var req_t = reg_str.ToHGlobal();
                    var req_h = req_t.Item1;
                    var req_l = req_t.Item2;
                    var res_h = yuki.request(req_h.ToPointer(), &req_l);
                    var res_p = new IntPtr(res_h);
                    ct.Register(() => Marshal.FreeHGlobal(res_p));
                    var res_str = res_p.ToUtf8String(req_l);
                    res_str.Is("SHIORI/3.0 204 No Content\r\nCharset: UTF-8\r\n\r\n");
                }
                // unload
                {
                    Log.Trace("unload");
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