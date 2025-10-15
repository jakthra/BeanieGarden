SELECT 
  t1.id as common_plant_id,
  GREATEST(
    similarity(t1.common_danish_name, {{ search_term }}),
    similarity(t1.common_english_name, {{ search_term }}),
    similarity(t1.description, {{ search_term }}),
  ) as max_similarity
FROM common_plant t1
WHERE 
  t1.common_danish_name % '{{ search_term }}' OR
  t1.common_english_name % '{{ search_term }}' OR
  t1.description % '{{ search_term }}'
ORDER BY max_similarity DESC
LIMIT 50;