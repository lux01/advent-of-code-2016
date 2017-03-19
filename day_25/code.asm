        ;; AKA d = a + 2550
        ;; From what I can tell, this part of the input is the only unique part
        ;; Note 2550 = c * b = 15 * 170
	cpy a d                 ; d = a
	cpy 15 c                ; c = 15
	cpy 170 b               ;              <| b = 170
	inc d                   ;               |              <| d++
	dec b                   ;               |               | b--
	jnz b -2                ;               | while b != 0 >|
	dec c                   ;               | c --
	jnz c -5                ; while c != 0 >|

        
	cpy d a                 ;            <| a = d = a-init + 2550
	jnz 0 0                 ;             |              <|

        ;; AKA b = 0, c = (if (even? a) 2 1), a = floor(a / 2)
	cpy a b                 ;             |               | b = a-init + 2550
	cpy 0 a                 ;             |               | a = 0
        jnz 0 0                 ;             |               |       |>
	cpy 2 c                 ;             |               |       | c = 2
	jnz b 2                 ;             |               |       | if b != 0 >|    <|
	jnz 1 6                 ;             |               |       | else       | >|  |
	dec b                   ;             |               |       | b--       <|  |  |
 	dec c                   ;             |               |       | c--           |  |
	jnz c -4                ;             |               |       | while c != 0  | >|
 	inc a                   ;             |               |       | a++           |
	jnz 1 -7                ;             |               | loop >|               |
        jnz 0 0                 ;             |               |                      <|

        ;; a = floor(a / 2)
        ;; b = 2
        ;; c = (if (even? a) 2 1)
        ;; 
        ;; output 0 if a even, 1 if a odd
        cpy 2 b                 ;             |               | b = 2                
	jnz c 2                 ;             |               |      <| if c != 0 >|
	jnz 1 4                 ;             |               |       | else       | >|
	dec b                   ;             |               |       | b--       <|  |
	dec c                   ;             |               |       | c--           |
	jnz 1 -4                ;             |               | loop >|               |
	jnz 0 0                 ;             |               |                      <| 
	out b                   ;             |               | OUTPUT b
	jnz a -19               ;             | while a != 0 >|
	jnz 1 -21               ; while true >|

        ;; Summary
        ;; 
        ;; Setup: a -> a + 2550
        ;; Loop does the following:
        ;;      Output 0 if a is even, 1 if a is odd
        ;;      a -> floor(a / 2)
        ;;
        ;; The loop resets if a reaches zero, which is only going to happen if
        ;; the value of a at the start of the loop is 1, which is odd so the output is 1.
        ;;
        ;; Values of a:
        ;; ... <- 1 <- 2 <- 5 <- 10 <- 21 <- 42 <- 85 <- ... <- start
        ;;
        ;; This sequence has the following form:
        ;;
        ;; a(n) = (2^(n+2) + (-1)^(n+1) - 3) / 6
        ;;
        ;; So the challenge is to find the minimal value of n such that
        ;; a(n) - 2550 >= 0
        ;; a_{init} = a(n) - 2550
        ;; 
        ;; This can be solved by simple iteration: n = 12, so a_{init} = 180
