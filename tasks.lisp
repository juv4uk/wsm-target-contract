; tasks.my — wsm-target-contract
; Neutral machine-readable target ABI. Consumers: cml, wsm-os-lisp, wsm-my-lisp.

((kind . tasks)
 (version . 1)
 (tasks .
  (("WSM-CONTRACT-TAG-BOXED" .
    ((priority . 9.7)
     (done . "ratified as Tag::Boxed=7 in contract v3; extended with game-handle in v4 and rational in v5")
     (origin . cyberpunk-wsm-my-lisp)
     (capabilities . (abi tags contract))
     (context . "wsm-my-lisp proposed TAG_STRING=7 then generalized to TAG_BOXED=7 (boxed/ref; String first payload). TENTATIVE until this contract ratifies. TAG_BITS=3 pressure: prefer one generic boxed tag over burning the last value on String-only.")
     (description . "Ratify or reject TAG_BOXED as the authoritative tag for variable-size side-table values (String now; Vector/NumericBuffer later without new primary tags). Write the decision into target-contract.lisp with a version field bump.")
     (acceptance . "One number, one name, documented; wsm-my-lisp and cml can drop TENTATIVE dual stories.")))
   ("WSM-CONTRACT-CYBERPUNK-NOTE" .
    ((priority . 7.0)
     (done . nil)
     (description . "README note: Cyberpunk embed uses the same word/tag ABI; no second tag space for the game host.")
     (acceptance . "Short bilingual paragraph in README."))))))
