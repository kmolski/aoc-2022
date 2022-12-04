	.section ".rodata"

	.align  8
.ERR_INFILE:
	.asciz  "ERROR: An input file is required!\n"

    .align  8
.FOPEN_MODE:
    .asciz  "r"

    .align  8
.SSCANF_FMT:
    .asciz  "%ld"

    .align  8
.PART1_FMT:
    .asciz  "Part 1: %ld\n"

    .align  8
.PART2_FMT:
    .asciz  "Part 2: %ld\n"

	.section ".data"

    .align  8
.LINE_PTR:
    .xword  0

    .align  8
.N_CHARS:
    .xword  0

    .align  8
.FOOD_CALORIES:
    .word   0

    .align  8
.CALORIE_SUMS:
    .fill   512 * 4  ! 32-bit ints

	.section ".text"

    .align  4
_reverse_cmp:
    save    %sp, -224, %sp

    ld      [%i0], %l0  ! int a
    ld      [%i1], %l1  ! int b
    sub     %l1, %l0, %i0

    ret
    restore

	.align  4
	.global main
main:
	save	%sp, -224, %sp
    ! Check if the input file name is available
    cmp     %i0, 2
    bl      _INVALID_ARGS
    ! Open input file for reading, save FILE pointer
    add     %i1, 8, %l0
    ldx     [%l0], %o0  ! char* argv[1]
    set     .FOPEN_MODE, %o1
    call    fopen, 2
    nop
    mov     %o0, %l0
    ! Setup current sum and count variables
    mov     0, %l1  ! int current_sum
    mov     0, %l6  ! int count

_PROCESS_LINE:
    ! Check if end-of-file was reached. If true, sort the calorie amounts
    mov     %l0, %o0
    call    feof, 1
    nop
    tst     %o0
    bnz     _SORT_CALORIES
    ! Read line from input file
    set     .LINE_PTR, %o0
    stx     %g0, [%o0]  ! char* line = NULL
    set     .N_CHARS, %o1
    stx     %g0, [%o1]  ! size_t n = 0
    mov     %l0, %o2
    call    getline, 3
    ! Extract number of calories from input line
    set     .LINE_PTR, %l3
    ldx     [%l3], %o0
    set     .SSCANF_FMT, %o1
    set     .FOOD_CALORIES, %o2
    call    sscanf, 3
    nop
    ! Check if line is empty. If true, save the calorie total for the elf
    cmp     %o0, 1
    bne     _NEXT_ELF

_ADD_CALORIES:
    set     .FOOD_CALORIES, %l3
    ldx     [%l3], %l2
    add     %l1, %l2, %l1
    ba      _FREE_LINE
    nop

_NEXT_ELF:
    set     .CALORIE_SUMS, %l4
    add     %l4, %l6, %l4
    st      %l1, [%l4]
    ! Increment the elf count & reset the current calorie sum
    inc     4, %l6
    mov     0, %l1

_FREE_LINE:
    set     .LINE_PTR, %l5
    ldx     [%l5], %o0  ! char* line
    call    free, 1
    nop
    ba      _PROCESS_LINE
    nop

_SORT_CALORIES:
    set     .CALORIE_SUMS, %o0
    mov     512, %o1
    mov     4, %o2
    set     _reverse_cmp, %o3
    call    qsort, 4
    nop
    ! Print the part 1 result
    set     .CALORIE_SUMS, %l3
    set     .PART1_FMT, %o0
    ld      [%l3], %o1
    call    printf, 2
    ! Print the part 2 result
    set     .CALORIE_SUMS, %l3
    set     .PART2_FMT, %o0
    ld      [%l3], %o1
    ld      [%l3 + 4], %l1
    add     %o1, %l1, %o1
    ld      [%l3 + 8], %l2
    add     %o1, %l2, %o1
    call    printf, 2
    nop
    ! Close the input file
    mov     %l0, %o0
    call    fclose
    nop
    ! Return 0 from main
    mov     0, %i0
_EXIT:
    ret
    restore

_INVALID_ARGS:
    set     .ERR_INFILE, %o0
    call    printf, 1
    mov     1, %i0
    ba      _EXIT
    nop
