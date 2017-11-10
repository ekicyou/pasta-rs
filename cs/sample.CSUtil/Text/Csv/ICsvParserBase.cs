namespace CSUtil.Text.Csv
{
    public interface ICsvParserBase
    {
        /// <summary>読込み行</summary>
        int LineRow { get; }

        /// <summary>CSV行</summary>
        int CsvRow { get; }
    }
}