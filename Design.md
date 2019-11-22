# Ruch Design Notes


## Basic Data Types

### String

* String
  - value: string

### List

* List
  - elems: []string

### Set

* Set
  - elems: []string

### Sorted Set

* Elem
  - value:  string
  - weight: int

* SortedSet
  - elems: []Elem

### Hash

* Elem
  - key:   string
  - value: string

* Hash
  - elems: []Elem


## Cache

* DictEntry
  - key:   string
  - value: T
  - next:  DictEntry

* Dict
  - table: []DictEntry

* Cache
  - partitions: []Dict


## Dump File

* Tag: `RUCH(4){RDUMP_VERSION}(4)`

* Partition: `{PARTITION_TYPE}(1){parition_id}(4)`
  - String: 
    - type:  `{STRING_TYPE}(1)`
    - key:   `{key_len}(4){key}(key_len)`
    - value: `{val_len}(4){val}(val_len)`
  - List:
    - type:  `{LIST_TYPE}(1)`
    - key:   `{key_len}(4){key}(key_len)`
    - value: `{elem_cnt}(4){val_n_len}(4){val_n}(val_n_len)...`
  - Set:
    - type:  `{SET_TYPE}(1)`
    - key:   `{key_len}(4){key}(key_len)`
    - value: `{elem_cnt}(4){val_n_len}(4){val_n}(val_n_len)...`
  - SortedSet:
    - type:  `{SORTED_SET_TYPE}(1)`
    - key:   `{key_len}(4){key}(key_len)`
    - value: `{elem_cnt}(4){val_n_weight}(4){val_n_len}(4){val_n}(val_n_len)...`
  - Hash:
    - type:  `{HASH_TYPE}(1)`
    - key:   `{key_len}(4){key}(key_len)`
    - value: `{elem_cnt}(4){ekey_n_len}(4){ekey_n}(ekey_n_len){eval_n_len}(4){eval_n}(eval_n_len)...`

* End: `{EOF_TYPE}(1)`

