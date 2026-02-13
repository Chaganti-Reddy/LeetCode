# Write your MySQL query statement below
SELECT DISTINCT author_id AS id FROM Views 
WHERE Views.author_id = Views.viewer_id
ORDER BY author_id ASC;