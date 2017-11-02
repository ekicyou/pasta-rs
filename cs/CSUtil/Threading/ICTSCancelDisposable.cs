namespace System.Threading
{
    public interface ICTSCancelDisposable : IDisposable
    {
        CancellationToken Token { get; }
    }
}