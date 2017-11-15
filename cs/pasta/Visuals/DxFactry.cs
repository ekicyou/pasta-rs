using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using WIC = SharpDX.WIC;

namespace Pasta.Visuals
{
    public static class DxFactry
    {
        private struct Fields
        {
            public Lazy<WIC.ImagingFactory> WIC;
        }

        private static Fields _ = new Fields
        {
            WIC = new Lazy<WIC.ImagingFactory>(() => new WIC.ImagingFactory()),
        };

        public static WIC.ImagingFactory WIC => _.WIC.Value;
    }
}