typedef ULONG WNF_CHANGE_STAMP, *PWNF_CHANGE_STAMP;

typedef struct _WNF_STATE_NAME
{
    ULONG Data[2];
} WNF_STATE_NAME, *PWNF_STATE_NAME;

typedef struct _WNF_TYPE_ID
{
    GUID TypeId;
} WNF_TYPE_ID, *PWNF_TYPE_ID;

typedef const WNF_TYPE_ID *PCWNF_TYPE_ID;

typedef NTSTATUS (*PWNF_USER_CALLBACK)(
    WNF_STATE_NAME StateName,
    WNF_CHANGE_STAMP ChangeStamp,
    PWNF_TYPE_ID TypeId,
    PVOID CallbackContext,
    PVOID Buffer,
    ULONG BufferSize);

NTSYSAPI
NTSTATUS
NTAPI
RtlQueryWnfStateData(
    _Out_ PWNF_CHANGE_STAMP ChangeStamp,
    _In_ WNF_STATE_NAME StateName,
    _In_ PWNF_USER_CALLBACK Callback,
    _In_opt_ PVOID CallbackContext,
    _In_opt_ PWNF_TYPE_ID TypeId);

NTSYSAPI
NTSTATUS
NTAPI
RtlPublishWnfStateData(
    _In_ WNF_STATE_NAME StateName,
    _In_opt_ PCWNF_TYPE_ID TypeId,
    _In_reads_bytes_opt_(Length) const VOID *Buffer,
    _In_opt_ ULONG Length,
    _In_opt_ const VOID *ExplicitScope);
