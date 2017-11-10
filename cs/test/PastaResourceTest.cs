using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace test
{
    [TestClass]
    public class PastaResourceTest
    {
        [TestMethod]
        public void リソース埋め込み状況()
        {
            var asm = typeof(Pasta.Hana).Assembly;
            foreach(var name in asm.GetManifestResourceNames())
            {
                System.Diagnostics.Debug.WriteLine(name);
            }

        }
    }
}
