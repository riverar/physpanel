using System;
using System.Runtime.InteropServices;
using Windows.Win32.Foundation;
using Windows.Win32.Foundation.Metadata;

namespace Microsoft.Internal
{
    [StructLayout(LayoutKind.Auto, CharSet = CharSet.Auto)]
    [UnmanagedFunctionPointer(CallingConvention.Winapi)]
    unsafe public delegate NTSTATUS PWNF_USER_CALLBACK(
        [In] WNF_STATE_NAME StateName,
        [In] uint ChangeStamp,
        [Optional][In] WNF_TYPE_ID* TypeId,
        [Optional][In] void* CallbackContext,
        [Optional][In][Const][MemorySize(BytesParamIndex = 5)] void* Buffer,
        [In] uint Length);
}