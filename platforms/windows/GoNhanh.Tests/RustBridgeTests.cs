using Xunit;
using GoNhanh.Core;

namespace GoNhanh.Tests;

public class RustBridgeTests
{
    [Fact]
    public void TestInitialize()
    {
        RustBridge.Initialize();
        // Should not throw
    }

    [Fact]
    public void TestSetMethod()
    {
        RustBridge.Initialize();
        RustBridge.SetMethod(InputMethod.Telex);
        RustBridge.SetMethod(InputMethod.VNI);
    }

    [Fact]
    public void TestProcessKey()
    {
        RustBridge.Initialize();
        RustBridge.SetMethod(InputMethod.Telex);

        var result = RustBridge.ProcessKey((ushort)'A', false, false);
        Assert.Equal(ImeAction.None, result.Action);
    }

    [Fact]
    public void TestModernTone()
    {
        RustBridge.Initialize();
        // This will fail if ime_modern() is missing
        RustBridge.SetModernTone(true);
        RustBridge.SetModernTone(false);
    }

    [Fact]
    public void TestVietnameseInput()
    {
        RustBridge.Initialize();
        RustBridge.SetMethod(InputMethod.Telex);
        RustBridge.SetEnabled(true);

        // Type 'a' + 's' should produce 'á'
        RustBridge.ProcessKey((ushort)'A', false, false);
        var result = RustBridge.ProcessKey((ushort)'S', false, false);

        Assert.Equal(ImeAction.Send, result.Action);
        Assert.Equal(1, result.Backspace);
        Assert.Contains('á', result.GetText());
    }
}
