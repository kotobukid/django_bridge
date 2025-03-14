select id, name, code, feature_bits1 as f1, feature_bits2 as f2, skill_text, burst_text
from wix_card
where feature_bits1 = 0
  and feature_bits2 = 0
  and code LIKE 'WX24-P2%'
order by feature_bits1 ASC;

select id, name, code, feature_bits1 as f1, feature_bits2 as f2, skill_text, burst_text
from wix_card
where code LIKE 'WX24-P2%'
  and rarity != 'TK'
order by feature_bits1 ASC;


-- ENER(bits1, 20)
select id, name, code, feature_bits1 as f1, feature_bits2 as f2, skill_text, burst_text
from wix_card
where (feature_bits1 & (1 << 20)) <> 0
order by feature_bits1 ASC;

-- DECK BOUNCE(24) & POWER DOWN(22)
select id, name, code, feature_bits1 as f1, feature_bits2 as f2, skill_text, burst_text
from wix_card
where (feature_bits1 & (1 << 22)) <> 0
  and (feature_bits1 & (1 << 24)) <> 0
order by feature_bits1 ASC;

