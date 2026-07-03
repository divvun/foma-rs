# foma/io.c

> [spec:foma:def:io.binaryline]
> struct binaryline {
>   int type;
>   int state;
>   int in;
>   int target;
>   int out;
>   int symbol;
>   char *name;
>   char *value;
> }

> [spec:foma:def:io.bom]
> typedef struct BOM

> [spec:foma:def:io.check-bom-fn]
> BOM *check_BOM(char *buffer)

> [spec:foma:sem:io.check-bom-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.escape-print-fn]
> void escape_print(FILE *stream, char* string)

> [spec:foma:sem:io.escape-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.explode-line-fn]
> static INLINE int explode_line(char *buf, int *values)

> [spec:foma:sem:io.explode-line-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.file-to-mem-fn]
> char *file_to_mem(char *name)

> [spec:foma:sem:io.file-to-mem-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.foma-net-print-fn]
> int foma_net_print(struct fsm *net, gzFile outfile)

> [spec:foma:sem:io.foma-net-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.foma-write-prolog-fn]
> int foma_write_prolog (struct fsm *net, char *filename)

> [spec:foma:sem:io.foma-write-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.fsm-read-binary-file-fn]
> struct fsm *fsm_read_binary_file(char *filename)

> [spec:foma:sem:io.fsm-read-binary-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.fsm-read-binary-file-multiple-fn]
> struct fsm *fsm_read_binary_file_multiple(fsm_read_binary_handle fsrh)

> [spec:foma:sem:io.fsm-read-binary-file-multiple-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.fsm-read-binary-file-multiple-init-fn]
> fsm_read_binary_handle fsm_read_binary_file_multiple_init(char *filename)

> [spec:foma:sem:io.fsm-read-binary-file-multiple-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.fsm-read-prolog-fn]
> struct fsm *fsm_read_prolog (char *filename)

> [spec:foma:sem:io.fsm-read-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.fsm-read-spaced-text-file-fn]
> struct fsm *fsm_read_spaced_text_file(char *filename)

> [spec:foma:sem:io.fsm-read-spaced-text-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.fsm-read-text-file-fn]
> struct fsm *fsm_read_text_file(char *filename)

> [spec:foma:sem:io.fsm-read-text-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.fsm-write-binary-file-fn]
> int fsm_write_binary_file(struct fsm *net, char *filename)

> [spec:foma:sem:io.fsm-write-binary-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-buf-handle]
> struct io_buf_handle {
>   char *io_buf;
>   char *io_buf_ptr;
> }

> [spec:foma:def:io.io-free-fn]
> void io_free(struct io_buf_handle *iobh)

> [spec:foma:sem:io.io-free-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-get-file-size-fn]
> static size_t io_get_file_size(char *filename)

> [spec:foma:sem:io.io-get-file-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-get-gz-file-size-fn]
> static size_t io_get_gz_file_size(char *filename)

> [spec:foma:sem:io.io-get-gz-file-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-get-regular-file-size-fn]
> static size_t io_get_regular_file_size(char *filename)

> [spec:foma:sem:io.io-get-regular-file-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-gets-fn]
> static int io_gets(struct io_buf_handle *iobh, char *target)

> [spec:foma:sem:io.io-gets-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-gz-file-to-mem-fn]
> size_t io_gz_file_to_mem(struct io_buf_handle *iobh, char *filename)

> [spec:foma:sem:io.io-gz-file-to-mem-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-init-fn]
> struct io_buf_handle *io_init()

> [spec:foma:sem:io.io-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.io-net-read-fn]
> struct fsm *io_net_read(struct io_buf_handle *iobh, char **net_name)

> [spec:foma:sem:io.io-net-read-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.load-defined-fn]
> int load_defined(struct defined_networks *def, char *filename)

> [spec:foma:sem:io.load-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.net-print-att-fn]
> int net_print_att(struct fsm *net, FILE *outfile)

> [spec:foma:sem:io.net-print-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.read-att-fn]
> struct fsm *read_att(char *filename)

> [spec:foma:sem:io.read-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.save-defined-fn]
> int save_defined(struct defined_networks *def, char *filename)

> [spec:foma:sem:io.save-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.spacedtext-get-next-line-fn]
> char *spacedtext_get_next_line(char **text)

> [spec:foma:sem:io.spacedtext-get-next-line-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:io.spacedtext-get-next-token-fn]
> char *spacedtext_get_next_token(char **text)

> [spec:foma:sem:io.spacedtext-get-next-token-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

