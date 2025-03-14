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


-- BOUNCE(bits1, 22)
select id, name, code, feature_bits1 as f1, feature_bits2 as f2, skill_text, burst_text
from wix_card
where (feature_bits1 & (1 << 20)) <> 0
order by feature_bits1 ASC;

-- BOUNCE(22) & SALVAGE(24)
select id, name, code, feature_bits1 as f1, feature_bits2 as f2, skill_text, burst_text
from wix_card
where (feature_bits1 & (1 << 22)) <> 0
  and (feature_bits1 & (1 << 24)) <> 0
order by feature_bits1 ASC;

