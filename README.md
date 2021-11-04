# batch-reporter-rs

Simple command line tool to report on batches.

## Installation

TODO

## Usage

Provide an `.env` file in the working directory as such:

```shell
BASE_URL="https://api.example.org/rest"
MH_REST_USER="some_username"
MH_REST_PASSWD="a_secret_password"
```

(See: `.env.example`.)


Explore the command line options:

```shell
$ ./batch_reporter_rs --help

batch_reporter_rs 0.1.0

USAGE:
    batch_reporter_rs [OPTIONS] --batch_name <batch-name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --batch_name <batch-name>          The batch we want to report on
    -n, --nr_of_results <nr-of-results>    Nr of results to return from MH's REST API. Set this to a higher value if the
                                           batch contains more than 1000 records [default: 1000]
    -s, --search_by <search-by>            Search criterion: what parameter to search by. Defaults to `batch_id` but can
                                           be any indexed field in MediaHaven. Another option for batch is
                                           `dc_identifier_localidsbatch` [default: batch_id]
```

Report on a batch:

```shell
$ ./batch_reporter_rs -b some-batch-name-001
Searching for batch: "some-batch-name-001" (by `batch_id`)
Status: 200 OK

Got 3901 result(s) for batch: batch_id.
+---------------+-------+
| ArchiveStatus | count |
+---------------+-------+
| on_tape       |   900 |
+---------------+-------+
| on_disk       |    50 |
+---------------+-------+
| in_progress   |    30 |
+---------------+-------+
| failed        |    20 |
+---------------+-------+
```
