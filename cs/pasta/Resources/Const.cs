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
        }
    }
}