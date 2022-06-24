Grid ← {𝕩⥊(↕×´𝕩)∊𝕨•rand.Deal ×´𝕩}
Pad ← {0∾˘⍉⌽𝕩}⍟4
Adjacent ← +´∘⥊⎉2 3‿3↕Pad
Spread ← {⍉⌽𝕩∨»𝕩}⍟4
Flood ← ∧⟜Spread⍟{+´≢𝕩}
Neighbors←<⎉2 3‿3↕⊢

Game ← {count 𝕊 size:
  repr ← ".x 12345678ff"
  mines ← count Grid size
  adj ← Adjacent mines
  seen ← size⥊0
  flags ← size⥊0
  Clamp ← size⊸{h‿w 𝕊 y‿x: ⟨(h-1)(0⌈⌊)y, (w-1)(0⌈⌊)x⟩}
  Guess ⇐ {
    g ← 1⌾(𝕩⊸⊑)size⥊0
    seen ∨↩(0=adj)Spread∘Flood⍟(0=+´⥊g×adj) g×¬flags
    flags ×↩¬seen
  }
  Render  ⇐ {𝕊: ⊑⟜repr⎊'/'¨(flags×2-˜≠repr)+seen×seen+(¬mines)×1+adj}
  Result  ⇐ {𝕊: (seen≡¬mines) - ∨´⥊seen∧mines}
  Flag    ⇐ {flags ¬⌾(𝕩⊸⊑)⍟(0=⊑⟜seen 𝕩)↩}
  Chord   ⇐ {
    p←𝕩⊑adj×seen
    Valid←{(×p) ∧ p=+´+˝𝕩⊑Neighbors Pad flags}
    ChordCoords←⥊·∾⌜´((↕3)+1-˜⊢)¨
    (Guess∘Clamp¨ChordCoords)⍟Valid 𝕩
  }
  ShowAll ⇐ {𝕊: seen∨↩1}
}
