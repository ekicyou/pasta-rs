using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Reflection;
using System.IO;
using System.Runtime.CompilerServices;

namespace Pasta.Resources
{
    public static class Const
    {
        public static class Shell
        {
            private static Assembly MyAssembly => Assembly.GetCallingAssembly();
            private static Stream X([CallerMemberName]string name = null) => MyAssembly.GetManifestResourceStream($"Pasta.Resources.Shell.{name}.png");

            public static Stream _0 => X();
            public static Stream _1 => X();
            public static Stream _2 => X();
            public static Stream _3 => X();
            public static Stream _4 => X();
            public static Stream _5 => X();
            public static Stream _6 => X();
            public static Stream _base => X();
            public static Stream _icon => X();
            public static Stream x1 => X();
            public static Stream x2 => X();
            public static Stream x3 => X();
            public static Stream x4 => X();
            public static Stream x5 => X();
            public static Stream xyz => X();
            public static Stream y1 => X();
            public static Stream y2 => X();
            public static Stream y3 => X();
            public static Stream y4 => X();
            public static Stream y5 => X();
            public static Stream y6 => X();
            public static Stream z1 => X();
            public static Stream z2 => X();

            public static IEnumerable<Tuple<string, Stream>> AllItems => new[]
            {
                Tuple.Create(nameof(_0    ),_0 ),
                Tuple.Create(nameof(_1    ),_1 ),
                Tuple.Create(nameof(_2    ),_2 ),
                Tuple.Create(nameof(_3    ),_3 ),
                Tuple.Create(nameof(_4    ),_4 ),
                Tuple.Create(nameof(_5    ),_5 ),
                Tuple.Create(nameof(_6    ),_6 ),
                Tuple.Create(nameof(_base ),_base  ),
                Tuple.Create(nameof(_icon ),_icon  ),
                Tuple.Create(nameof(x1    ),x1 ),
                Tuple.Create(nameof(x2    ),x2 ),
                Tuple.Create(nameof(x3    ),x3 ),
                Tuple.Create(nameof(x4    ),x4 ),
                Tuple.Create(nameof(x5    ),x5 ),
                Tuple.Create(nameof(xyz   ),xyz ),
                Tuple.Create(nameof(y1    ),y1 ),
                Tuple.Create(nameof(y2    ),y2 ),
                Tuple.Create(nameof(y3    ),y3 ),
                Tuple.Create(nameof(y4    ),y4 ),
                Tuple.Create(nameof(y5    ),y5 ),
                Tuple.Create(nameof(y6    ),y6 ),
                Tuple.Create(nameof(z1    ),z1 ),
                Tuple.Create(nameof(z2    ),z2 ),
            };
        }
    }
}