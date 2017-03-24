/* automatically generated by rust-bindgen */

pub const ZSTD_VERSION_MAJOR: ::libc::c_uint = 1;
pub const ZSTD_VERSION_MINOR: ::libc::c_uint = 1;
pub const ZSTD_VERSION_RELEASE: ::libc::c_uint = 4;
pub const ZSTD_MAGICNUMBER: ::libc::c_uint = 4247762216;
pub const ZSTD_MAGIC_SKIPPABLE_START: ::libc::c_uint = 407710288;
pub const ZSTD_CONTENTSIZE_UNKNOWN: ::libc::c_int = -1;
pub const ZSTD_CONTENTSIZE_ERROR: ::libc::c_int = -2;
pub const ZSTD_WINDOWLOG_MAX_32: ::libc::c_uint = 27;
pub const ZSTD_WINDOWLOG_MAX_64: ::libc::c_uint = 27;
pub const ZSTD_WINDOWLOG_MIN: ::libc::c_uint = 10;
pub const ZSTD_HASHLOG_MIN: ::libc::c_uint = 6;
pub const ZSTD_CHAINLOG_MIN: ::libc::c_uint = 6;
pub const ZSTD_HASHLOG3_MAX: ::libc::c_uint = 17;
pub const ZSTD_SEARCHLOG_MIN: ::libc::c_uint = 1;
pub const ZSTD_SEARCHLENGTH_MAX: ::libc::c_uint = 7;
pub const ZSTD_SEARCHLENGTH_MIN: ::libc::c_uint = 3;
pub const ZSTD_TARGETLENGTH_MIN: ::libc::c_uint = 4;
pub const ZSTD_TARGETLENGTH_MAX: ::libc::c_uint = 999;
pub const ZSTD_FRAMEHEADERSIZE_MAX: ::libc::c_uint = 18;
pub const ZSTD_FRAMEHEADERSIZE_MIN: ::libc::c_uint = 6;
pub const ZSTD_BLOCKSIZE_ABSOLUTEMAX: ::libc::c_uint = 131072;
pub type wchar_t = ::libc::c_int;
extern "C" {
    pub fn ZSTD_versionNumber() -> ::libc::c_uint;
}
extern "C" {
    pub fn ZSTD_compress(dst: *mut ::libc::c_void, dstCapacity: usize,
                         src: *const ::libc::c_void, srcSize: usize,
                         compressionLevel: ::libc::c_int) -> usize;
}
extern "C" {
    pub fn ZSTD_decompress(dst: *mut ::libc::c_void, dstCapacity: usize,
                           src: *const ::libc::c_void, compressedSize: usize)
     -> usize;
}
extern "C" {
    pub fn ZSTD_getDecompressedSize(src: *const ::libc::c_void,
                                    srcSize: usize) -> ::libc::c_ulonglong;
}
extern "C" {
    pub fn ZSTD_maxCLevel() -> ::libc::c_int;
}
extern "C" {
    pub fn ZSTD_compressBound(srcSize: usize) -> usize;
}
extern "C" {
    pub fn ZSTD_isError(code: usize) -> ::libc::c_uint;
}
extern "C" {
    pub fn ZSTD_getErrorName(code: usize) -> *const ::libc::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_CCtx_s([u8; 0]);
pub type ZSTD_CCtx = ZSTD_CCtx_s;
extern "C" {
    pub fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
}
extern "C" {
    pub fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_compressCCtx(ctx: *mut ZSTD_CCtx, dst: *mut ::libc::c_void,
                             dstCapacity: usize, src: *const ::libc::c_void,
                             srcSize: usize, compressionLevel: ::libc::c_int)
     -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_DCtx_s([u8; 0]);
pub type ZSTD_DCtx = ZSTD_DCtx_s;
extern "C" {
    pub fn ZSTD_createDCtx() -> *mut ZSTD_DCtx;
}
extern "C" {
    pub fn ZSTD_freeDCtx(dctx: *mut ZSTD_DCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_decompressDCtx(ctx: *mut ZSTD_DCtx, dst: *mut ::libc::c_void,
                               dstCapacity: usize, src: *const ::libc::c_void,
                               srcSize: usize) -> usize;
}
extern "C" {
    pub fn ZSTD_compress_usingDict(ctx: *mut ZSTD_CCtx,
                                   dst: *mut ::libc::c_void,
                                   dstCapacity: usize,
                                   src: *const ::libc::c_void, srcSize: usize,
                                   dict: *const ::libc::c_void,
                                   dictSize: usize,
                                   compressionLevel: ::libc::c_int) -> usize;
}
extern "C" {
    pub fn ZSTD_decompress_usingDict(dctx: *mut ZSTD_DCtx,
                                     dst: *mut ::libc::c_void,
                                     dstCapacity: usize,
                                     src: *const ::libc::c_void,
                                     srcSize: usize,
                                     dict: *const ::libc::c_void,
                                     dictSize: usize) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_CDict_s([u8; 0]);
pub type ZSTD_CDict = ZSTD_CDict_s;
extern "C" {
    pub fn ZSTD_createCDict(dictBuffer: *const ::libc::c_void,
                            dictSize: usize, compressionLevel: ::libc::c_int)
     -> *mut ZSTD_CDict;
}
extern "C" {
    pub fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> usize;
}
extern "C" {
    pub fn ZSTD_compress_usingCDict(cctx: *mut ZSTD_CCtx,
                                    dst: *mut ::libc::c_void,
                                    dstCapacity: usize,
                                    src: *const ::libc::c_void,
                                    srcSize: usize, cdict: *const ZSTD_CDict)
     -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_DDict_s([u8; 0]);
pub type ZSTD_DDict = ZSTD_DDict_s;
extern "C" {
    pub fn ZSTD_createDDict(dictBuffer: *const ::libc::c_void,
                            dictSize: usize) -> *mut ZSTD_DDict;
}
extern "C" {
    pub fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> usize;
}
extern "C" {
    pub fn ZSTD_decompress_usingDDict(dctx: *mut ZSTD_DCtx,
                                      dst: *mut ::libc::c_void,
                                      dstCapacity: usize,
                                      src: *const ::libc::c_void,
                                      srcSize: usize,
                                      ddict: *const ZSTD_DDict) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ZSTD_inBuffer_s {
    pub src: *const ::libc::c_void,
    pub size: usize,
    pub pos: usize,
}
#[test]
fn bindgen_test_layout_ZSTD_inBuffer_s() {
    assert_eq!(::core::mem::size_of::<ZSTD_inBuffer_s>() , 24usize , concat !
               ( "Size of: " , stringify ! ( ZSTD_inBuffer_s ) ));
    assert_eq! (::core::mem::align_of::<ZSTD_inBuffer_s>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( ZSTD_inBuffer_s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_inBuffer_s ) ) . src as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_inBuffer_s ) ,
                "::" , stringify ! ( src ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_inBuffer_s ) ) . size as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_inBuffer_s ) ,
                "::" , stringify ! ( size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_inBuffer_s ) ) . pos as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_inBuffer_s ) ,
                "::" , stringify ! ( pos ) ));
}
impl Clone for ZSTD_inBuffer_s {
    fn clone(&self) -> Self { *self }
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut ::libc::c_void,
    pub size: usize,
    pub pos: usize,
}
#[test]
fn bindgen_test_layout_ZSTD_outBuffer_s() {
    assert_eq!(::core::mem::size_of::<ZSTD_outBuffer_s>() , 24usize , concat !
               ( "Size of: " , stringify ! ( ZSTD_outBuffer_s ) ));
    assert_eq! (::core::mem::align_of::<ZSTD_outBuffer_s>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( ZSTD_outBuffer_s ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_outBuffer_s ) ) . dst as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_outBuffer_s ) ,
                "::" , stringify ! ( dst ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_outBuffer_s ) ) . size as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_outBuffer_s ) ,
                "::" , stringify ! ( size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_outBuffer_s ) ) . pos as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_outBuffer_s ) ,
                "::" , stringify ! ( pos ) ));
}
impl Clone for ZSTD_outBuffer_s {
    fn clone(&self) -> Self { *self }
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_CStream_s([u8; 0]);
pub type ZSTD_CStream = ZSTD_CStream_s;
extern "C" {
    pub fn ZSTD_createCStream() -> *mut ZSTD_CStream;
}
extern "C" {
    pub fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> usize;
}
extern "C" {
    pub fn ZSTD_initCStream(zcs: *mut ZSTD_CStream,
                            compressionLevel: ::libc::c_int) -> usize;
}
extern "C" {
    pub fn ZSTD_compressStream(zcs: *mut ZSTD_CStream,
                               output: *mut ZSTD_outBuffer,
                               input: *mut ZSTD_inBuffer) -> usize;
}
extern "C" {
    pub fn ZSTD_flushStream(zcs: *mut ZSTD_CStream,
                            output: *mut ZSTD_outBuffer) -> usize;
}
extern "C" {
    pub fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> usize;
}
extern "C" {
    pub fn ZSTD_CStreamInSize() -> usize;
}
extern "C" {
    pub fn ZSTD_CStreamOutSize() -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_DStream_s([u8; 0]);
pub type ZSTD_DStream = ZSTD_DStream_s;
extern "C" {
    pub fn ZSTD_createDStream() -> *mut ZSTD_DStream;
}
extern "C" {
    pub fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> usize;
}
extern "C" {
    pub fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> usize;
}
extern "C" {
    pub fn ZSTD_decompressStream(zds: *mut ZSTD_DStream,
                                 output: *mut ZSTD_outBuffer,
                                 input: *mut ZSTD_inBuffer) -> usize;
}
extern "C" {
    pub fn ZSTD_DStreamInSize() -> usize;
}
extern "C" {
    pub fn ZSTD_DStreamOutSize() -> usize;
}
pub const ZSTD_frameHeaderSize_prefix: usize = 5;
pub const ZSTD_frameHeaderSize_min: usize = 6;
pub const ZSTD_frameHeaderSize_max: usize = 18;
pub const ZSTD_skippableHeaderSize: usize = 8;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZSTD_strategy {
    ZSTD_fast = 0,
    ZSTD_dfast = 1,
    ZSTD_greedy = 2,
    ZSTD_lazy = 3,
    ZSTD_lazy2 = 4,
    ZSTD_btlazy2 = 5,
    ZSTD_btopt = 6,
    ZSTD_btopt2 = 7,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ZSTD_compressionParameters {
    pub windowLog: ::libc::c_uint,
    pub chainLog: ::libc::c_uint,
    pub hashLog: ::libc::c_uint,
    pub searchLog: ::libc::c_uint,
    pub searchLength: ::libc::c_uint,
    pub targetLength: ::libc::c_uint,
    pub strategy: ZSTD_strategy,
}
#[test]
fn bindgen_test_layout_ZSTD_compressionParameters() {
    assert_eq!(::core::mem::size_of::<ZSTD_compressionParameters>() , 28usize
               , concat ! (
               "Size of: " , stringify ! ( ZSTD_compressionParameters ) ));
    assert_eq! (::core::mem::align_of::<ZSTD_compressionParameters>() , 4usize
                , concat ! (
                "Alignment of " , stringify ! ( ZSTD_compressionParameters )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_compressionParameters ) ) .
                windowLog as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                ZSTD_compressionParameters ) , "::" , stringify ! ( windowLog
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_compressionParameters ) ) . chainLog
                as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! (
                ZSTD_compressionParameters ) , "::" , stringify ! ( chainLog )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_compressionParameters ) ) . hashLog
                as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                ZSTD_compressionParameters ) , "::" , stringify ! ( hashLog )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_compressionParameters ) ) .
                searchLog as * const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! (
                ZSTD_compressionParameters ) , "::" , stringify ! ( searchLog
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_compressionParameters ) ) .
                searchLength as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! (
                ZSTD_compressionParameters ) , "::" , stringify ! (
                searchLength ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_compressionParameters ) ) .
                targetLength as * const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! (
                ZSTD_compressionParameters ) , "::" , stringify ! (
                targetLength ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_compressionParameters ) ) . strategy
                as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! (
                ZSTD_compressionParameters ) , "::" , stringify ! ( strategy )
                ));
}
impl Clone for ZSTD_compressionParameters {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: ::libc::c_uint,
    pub checksumFlag: ::libc::c_uint,
    pub noDictIDFlag: ::libc::c_uint,
}
#[test]
fn bindgen_test_layout_ZSTD_frameParameters() {
    assert_eq!(::core::mem::size_of::<ZSTD_frameParameters>() , 12usize ,
               concat ! ( "Size of: " , stringify ! ( ZSTD_frameParameters )
               ));
    assert_eq! (::core::mem::align_of::<ZSTD_frameParameters>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( ZSTD_frameParameters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_frameParameters ) ) .
                contentSizeFlag as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_frameParameters )
                , "::" , stringify ! ( contentSizeFlag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_frameParameters ) ) . checksumFlag
                as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_frameParameters )
                , "::" , stringify ! ( checksumFlag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_frameParameters ) ) . noDictIDFlag
                as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_frameParameters )
                , "::" , stringify ! ( noDictIDFlag ) ));
}
impl Clone for ZSTD_frameParameters {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
#[test]
fn bindgen_test_layout_ZSTD_parameters() {
    assert_eq!(::core::mem::size_of::<ZSTD_parameters>() , 40usize , concat !
               ( "Size of: " , stringify ! ( ZSTD_parameters ) ));
    assert_eq! (::core::mem::align_of::<ZSTD_parameters>() , 4usize , concat !
                ( "Alignment of " , stringify ! ( ZSTD_parameters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_parameters ) ) . cParams as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_parameters ) ,
                "::" , stringify ! ( cParams ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_parameters ) ) . fParams as * const
                _ as usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_parameters ) ,
                "::" , stringify ! ( fParams ) ));
}
impl Clone for ZSTD_parameters {
    fn clone(&self) -> Self { *self }
}
pub type ZSTD_allocFunction =
    ::core::option::Option<unsafe extern "C" fn(opaque: *mut ::libc::c_void,
                                                size: usize)
                               -> *mut ::libc::c_void>;
pub type ZSTD_freeFunction =
    ::core::option::Option<unsafe extern "C" fn(opaque: *mut ::libc::c_void,
                                                address:
                                                    *mut ::libc::c_void)>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut ::libc::c_void,
}
#[test]
fn bindgen_test_layout_ZSTD_customMem() {
    assert_eq!(::core::mem::size_of::<ZSTD_customMem>() , 24usize , concat ! (
               "Size of: " , stringify ! ( ZSTD_customMem ) ));
    assert_eq! (::core::mem::align_of::<ZSTD_customMem>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( ZSTD_customMem ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_customMem ) ) . customAlloc as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_customMem ) , "::"
                , stringify ! ( customAlloc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_customMem ) ) . customFree as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_customMem ) , "::"
                , stringify ! ( customFree ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_customMem ) ) . opaque as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_customMem ) , "::"
                , stringify ! ( opaque ) ));
}
impl Clone for ZSTD_customMem {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn ZSTD_findFrameCompressedSize(src: *const ::libc::c_void,
                                        srcSize: usize) -> usize;
}
extern "C" {
    pub fn ZSTD_getFrameContentSize(src: *const ::libc::c_void,
                                    srcSize: usize) -> ::libc::c_ulonglong;
}
extern "C" {
    pub fn ZSTD_findDecompressedSize(src: *const ::libc::c_void,
                                     srcSize: usize) -> ::libc::c_ulonglong;
}
extern "C" {
    pub fn ZSTD_estimateCCtxSize(cParams: ZSTD_compressionParameters)
     -> usize;
}
extern "C" {
    pub fn ZSTD_createCCtx_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_CCtx;
}
extern "C" {
    pub fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> usize;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZSTD_CCtxParameter {
    ZSTD_p_forceWindow = 0,
    ZSTD_p_forceRawDict = 1,
}
extern "C" {
    pub fn ZSTD_setCCtxParameter(cctx: *mut ZSTD_CCtx,
                                 param: ZSTD_CCtxParameter,
                                 value: ::libc::c_uint) -> usize;
}
extern "C" {
    pub fn ZSTD_createCDict_byReference(dictBuffer: *const ::libc::c_void,
                                        dictSize: usize,
                                        compressionLevel: ::libc::c_int)
     -> *mut ZSTD_CDict;
}
extern "C" {
    pub fn ZSTD_createCDict_advanced(dict: *const ::libc::c_void,
                                     dictSize: usize,
                                     byReference: ::libc::c_uint,
                                     params: ZSTD_parameters,
                                     customMem: ZSTD_customMem)
     -> *mut ZSTD_CDict;
}
extern "C" {
    pub fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> usize;
}
extern "C" {
    pub fn ZSTD_getCParams(compressionLevel: ::libc::c_int,
                           estimatedSrcSize: ::libc::c_ulonglong,
                           dictSize: usize) -> ZSTD_compressionParameters;
}
extern "C" {
    pub fn ZSTD_getParams(compressionLevel: ::libc::c_int,
                          estimatedSrcSize: ::libc::c_ulonglong,
                          dictSize: usize) -> ZSTD_parameters;
}
extern "C" {
    pub fn ZSTD_checkCParams(params: ZSTD_compressionParameters) -> usize;
}
extern "C" {
    pub fn ZSTD_adjustCParams(cPar: ZSTD_compressionParameters,
                              srcSize: ::libc::c_ulonglong, dictSize: usize)
     -> ZSTD_compressionParameters;
}
extern "C" {
    pub fn ZSTD_compress_advanced(ctx: *mut ZSTD_CCtx,
                                  dst: *mut ::libc::c_void,
                                  dstCapacity: usize,
                                  src: *const ::libc::c_void, srcSize: usize,
                                  dict: *const ::libc::c_void,
                                  dictSize: usize, params: ZSTD_parameters)
     -> usize;
}
extern "C" {
    pub fn ZSTD_isFrame(buffer: *const ::libc::c_void, size: usize)
     -> ::libc::c_uint;
}
extern "C" {
    pub fn ZSTD_estimateDCtxSize() -> usize;
}
extern "C" {
    pub fn ZSTD_createDCtx_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_DCtx;
}
extern "C" {
    pub fn ZSTD_sizeof_DCtx(dctx: *const ZSTD_DCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_createDDict_byReference(dictBuffer: *const ::libc::c_void,
                                        dictSize: usize) -> *mut ZSTD_DDict;
}
extern "C" {
    pub fn ZSTD_createDDict_advanced(dict: *const ::libc::c_void,
                                     dictSize: usize,
                                     byReference: ::libc::c_uint,
                                     customMem: ZSTD_customMem)
     -> *mut ZSTD_DDict;
}
extern "C" {
    pub fn ZSTD_sizeof_DDict(ddict: *const ZSTD_DDict) -> usize;
}
extern "C" {
    pub fn ZSTD_getDictID_fromDict(dict: *const ::libc::c_void,
                                   dictSize: usize) -> ::libc::c_uint;
}
extern "C" {
    pub fn ZSTD_getDictID_fromDDict(ddict: *const ZSTD_DDict)
     -> ::libc::c_uint;
}
extern "C" {
    pub fn ZSTD_getDictID_fromFrame(src: *const ::libc::c_void,
                                    srcSize: usize) -> ::libc::c_uint;
}
extern "C" {
    pub fn ZSTD_createCStream_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_CStream;
}
extern "C" {
    pub fn ZSTD_initCStream_srcSize(zcs: *mut ZSTD_CStream,
                                    compressionLevel: ::libc::c_int,
                                    pledgedSrcSize: ::libc::c_ulonglong)
     -> usize;
}
extern "C" {
    pub fn ZSTD_initCStream_usingDict(zcs: *mut ZSTD_CStream,
                                      dict: *const ::libc::c_void,
                                      dictSize: usize,
                                      compressionLevel: ::libc::c_int)
     -> usize;
}
extern "C" {
    pub fn ZSTD_initCStream_advanced(zcs: *mut ZSTD_CStream,
                                     dict: *const ::libc::c_void,
                                     dictSize: usize, params: ZSTD_parameters,
                                     pledgedSrcSize: ::libc::c_ulonglong)
     -> usize;
}
extern "C" {
    pub fn ZSTD_initCStream_usingCDict(zcs: *mut ZSTD_CStream,
                                       cdict: *const ZSTD_CDict) -> usize;
}
extern "C" {
    pub fn ZSTD_resetCStream(zcs: *mut ZSTD_CStream,
                             pledgedSrcSize: ::libc::c_ulonglong) -> usize;
}
extern "C" {
    pub fn ZSTD_sizeof_CStream(zcs: *const ZSTD_CStream) -> usize;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZSTD_DStreamParameter_e { DStream_p_maxWindowSize = 0, }
extern "C" {
    pub fn ZSTD_createDStream_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_DStream;
}
extern "C" {
    pub fn ZSTD_initDStream_usingDict(zds: *mut ZSTD_DStream,
                                      dict: *const ::libc::c_void,
                                      dictSize: usize) -> usize;
}
extern "C" {
    pub fn ZSTD_setDStreamParameter(zds: *mut ZSTD_DStream,
                                    paramType: ZSTD_DStreamParameter_e,
                                    paramValue: ::libc::c_uint) -> usize;
}
extern "C" {
    pub fn ZSTD_initDStream_usingDDict(zds: *mut ZSTD_DStream,
                                       ddict: *const ZSTD_DDict) -> usize;
}
extern "C" {
    pub fn ZSTD_resetDStream(zds: *mut ZSTD_DStream) -> usize;
}
extern "C" {
    pub fn ZSTD_sizeof_DStream(zds: *const ZSTD_DStream) -> usize;
}
extern "C" {
    pub fn ZSTD_compressBegin(cctx: *mut ZSTD_CCtx,
                              compressionLevel: ::libc::c_int) -> usize;
}
extern "C" {
    pub fn ZSTD_compressBegin_usingDict(cctx: *mut ZSTD_CCtx,
                                        dict: *const ::libc::c_void,
                                        dictSize: usize,
                                        compressionLevel: ::libc::c_int)
     -> usize;
}
extern "C" {
    pub fn ZSTD_compressBegin_advanced(cctx: *mut ZSTD_CCtx,
                                       dict: *const ::libc::c_void,
                                       dictSize: usize,
                                       params: ZSTD_parameters,
                                       pledgedSrcSize: ::libc::c_ulonglong)
     -> usize;
}
extern "C" {
    pub fn ZSTD_copyCCtx(cctx: *mut ZSTD_CCtx, preparedCCtx: *const ZSTD_CCtx,
                         pledgedSrcSize: ::libc::c_ulonglong) -> usize;
}
extern "C" {
    pub fn ZSTD_compressBegin_usingCDict(cctx: *mut ZSTD_CCtx,
                                         cdict: *const ZSTD_CDict,
                                         pledgedSrcSize: ::libc::c_ulonglong)
     -> usize;
}
extern "C" {
    pub fn ZSTD_compressContinue(cctx: *mut ZSTD_CCtx,
                                 dst: *mut ::libc::c_void, dstCapacity: usize,
                                 src: *const ::libc::c_void, srcSize: usize)
     -> usize;
}
extern "C" {
    pub fn ZSTD_compressEnd(cctx: *mut ZSTD_CCtx, dst: *mut ::libc::c_void,
                            dstCapacity: usize, src: *const ::libc::c_void,
                            srcSize: usize) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ZSTD_frameParams {
    pub frameContentSize: ::libc::c_ulonglong,
    pub windowSize: ::libc::c_uint,
    pub dictID: ::libc::c_uint,
    pub checksumFlag: ::libc::c_uint,
}
#[test]
fn bindgen_test_layout_ZSTD_frameParams() {
    assert_eq!(::core::mem::size_of::<ZSTD_frameParams>() , 24usize , concat !
               ( "Size of: " , stringify ! ( ZSTD_frameParams ) ));
    assert_eq! (::core::mem::align_of::<ZSTD_frameParams>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( ZSTD_frameParams ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_frameParams ) ) . frameContentSize
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_frameParams ) ,
                "::" , stringify ! ( frameContentSize ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_frameParams ) ) . windowSize as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_frameParams ) ,
                "::" , stringify ! ( windowSize ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_frameParams ) ) . dictID as * const
                _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_frameParams ) ,
                "::" , stringify ! ( dictID ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ZSTD_frameParams ) ) . checksumFlag as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( ZSTD_frameParams ) ,
                "::" , stringify ! ( checksumFlag ) ));
}
impl Clone for ZSTD_frameParams {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn ZSTD_getFrameParams(fparamsPtr: *mut ZSTD_frameParams,
                               src: *const ::libc::c_void, srcSize: usize)
     -> usize;
}
extern "C" {
    pub fn ZSTD_decompressBegin(dctx: *mut ZSTD_DCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_decompressBegin_usingDict(dctx: *mut ZSTD_DCtx,
                                          dict: *const ::libc::c_void,
                                          dictSize: usize) -> usize;
}
extern "C" {
    pub fn ZSTD_copyDCtx(dctx: *mut ZSTD_DCtx,
                         preparedDCtx: *const ZSTD_DCtx);
}
extern "C" {
    pub fn ZSTD_nextSrcSizeToDecompress(dctx: *mut ZSTD_DCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_decompressContinue(dctx: *mut ZSTD_DCtx,
                                   dst: *mut ::libc::c_void,
                                   dstCapacity: usize,
                                   src: *const ::libc::c_void, srcSize: usize)
     -> usize;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZSTD_nextInputType_e {
    ZSTDnit_frameHeader = 0,
    ZSTDnit_blockHeader = 1,
    ZSTDnit_block = 2,
    ZSTDnit_lastBlock = 3,
    ZSTDnit_checksum = 4,
    ZSTDnit_skippableFrame = 5,
}
extern "C" {
    pub fn ZSTD_nextInputType(dctx: *mut ZSTD_DCtx) -> ZSTD_nextInputType_e;
}
extern "C" {
    pub fn ZSTD_getBlockSizeMax(cctx: *mut ZSTD_CCtx) -> usize;
}
extern "C" {
    pub fn ZSTD_compressBlock(cctx: *mut ZSTD_CCtx, dst: *mut ::libc::c_void,
                              dstCapacity: usize, src: *const ::libc::c_void,
                              srcSize: usize) -> usize;
}
extern "C" {
    pub fn ZSTD_decompressBlock(dctx: *mut ZSTD_DCtx,
                                dst: *mut ::libc::c_void, dstCapacity: usize,
                                src: *const ::libc::c_void, srcSize: usize)
     -> usize;
}
extern "C" {
    pub fn ZSTD_insertBlock(dctx: *mut ZSTD_DCtx,
                            blockStart: *const ::libc::c_void,
                            blockSize: usize) -> usize;
}
extern "C" {
    pub fn ZDICT_trainFromBuffer(dictBuffer: *mut ::libc::c_void,
                                 dictBufferCapacity: usize,
                                 samplesBuffer: *const ::libc::c_void,
                                 samplesSizes: *const usize,
                                 nbSamples: ::libc::c_uint) -> usize;
}
extern "C" {
    pub fn ZDICT_getDictID(dictBuffer: *const ::libc::c_void, dictSize: usize)
     -> ::libc::c_uint;
}
extern "C" {
    pub fn ZDICT_isError(errorCode: usize) -> ::libc::c_uint;
}
extern "C" {
    pub fn ZDICT_getErrorName(errorCode: usize) -> *const ::libc::c_char;
}
