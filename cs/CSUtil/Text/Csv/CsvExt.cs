using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace CSUtil.Text.Csv
{
    public static class CsvExt
    {
        private static Encoding CsvEncoding => Encoding.Default;

        public static ICsvAsyncReader CreateAsyncReader(string path, CancellationToken srcCT)
        {
            IDisposable d = null;
            try
            {
                var cts = new CancellationTokenSource().Begin();
                var reg = srcCT.Register(() => cts.Dispose());
                d = cts;
                var ct = cts.Token;
                ct.Register(() => reg.Dispose());

                var st = new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read)
                    .RegisterBy(ct);
                var r = new StreamReader(st, CsvEncoding)
                    .RegisterBy(ct);

                // 正常終了
                ct.Register(() => reg.Dispose());
                d = null;
                return new CsvAsyncReader(cts, r);
            }
            finally
            {
                if (d != null) d.Dispose();
            }
        }

        /// <summary>
        /// 入力されたtextをcsvとして列挙します。
        /// </summary>
        /// <param name="text"></param>
        /// <returns></returns>
        public static IEnumerable<string[]> ParseText(string text)
        {
            using (var r = new StringReader(text))
            {
                var csvRead = new CsvAsyncReader(null, r);
                while (true)
                {
                    var item = csvRead.ReadCsvLine();
                    if (item == null) yield break;
                    yield return item;
                }
            }
        }

        private class CsvAsyncReader : ICsvReader, ICsvAsyncReader
        {
            private CsvPushParser Parser { get; } = new CsvPushParser();
            public int CsvRow => Parser.CsvRow;
            public int LineRow => Parser.LineRow;
            private IDisposable Disp { get; set; }
            private TextReader Reader { get; }

            public void Dispose()
            {
                var d = Disp;
                Disp = null;
                if (d == null) return;
                d.Dispose();
            }

            public CsvAsyncReader(IDisposable disp, TextReader reader)
            {
                Disp = disp;
                Reader = reader;
            }

            public async Task<string[]> ReadCsvLineAsync(CancellationToken ct)
            {
                while (Reader.Peek() != -1)
                {
                    ct.ThrowIfCancellationRequested();
                    var line = await Reader.ReadLineAsync();
                    var csv = Parser.PushLine(line);
                    if (csv != null) return csv;
                }
                return null;
            }

            public string[] ReadCsvLine()
            {
                while (Reader.Peek() != -1)
                {
                    var line = Reader.ReadLine();
                    var csv = Parser.PushLine(line);
                    if (csv != null) return csv;
                }
                return null;
            }
        }
    }
}