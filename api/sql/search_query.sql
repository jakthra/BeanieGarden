SELECT
  t1.gbif_genus_key as gbif_genus_key,
  t1.id,
  GREATEST(
    similarity(t1.common_danish_name, {{ search_term }}),
    similarity(t1.common_english_name, {{ search_term }}),
    similarity(t1.description, {{ search_term }}),
  ) as max_similarity
FROM common_plant t1
ORDER BY max_similarity DESC
LIMIT 50;