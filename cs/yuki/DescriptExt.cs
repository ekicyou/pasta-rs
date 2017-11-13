using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

namespace Setugekka.Yuki
{
    public static class DescriptExt
    {
        public static IDictionary<string, string> Read(string path)
        {
            try
            {
                return EnRead(path, Encoding.Default)
                    .Distinct(new KeyComp())
                    .ToDictionary(a => a.Key, a => a.Value);
            }
            catch
            {
                return new Dictionary<string, string>();
            }
        }

        private class KeyComp : IEqualityComparer<KeyValuePair<string, string>>
        {
            public bool Equals(KeyValuePair<string, string> x, KeyValuePair<string, string> y)
            {
                return x.Key == y.Key;
            }

            public int GetHashCode(KeyValuePair<string, string> obj)
            {
                return obj.Key.GetHashCode();
            }
        }

        private static IEnumerable<KeyValuePair<string, string>> EnRead(string path, Encoding enc)
        {
            using (var s = new FileStream(path, FileMode.Open, FileAccess.Read))
            using (var r = new StreamReader(s, enc))
            {
                while (!r.EndOfStream)
                {
                    var line = r.ReadLine();
                    var m = ReLine.Match(line);
                    if (!m.Success) continue;
                    var key = m.Groups["key"].Value.Trim();
                    var value = m.Groups["value"].Value.Trim();
                    yield return new KeyValuePair<string, string>(key, value);
                }
            }
        }

        private static Regex ReLine { get; } = new Regex(@"^(?<key>[^,]*),(?<value>.*)$", RegexOptions.Compiled);

        public static string GetOrDefault(this IDictionary<string, string> dic, string key, string def)
        {
            if (!dic.ContainsKey(key)) return def;
            return dic[key];
        }
    }
}