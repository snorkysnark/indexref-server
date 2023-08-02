WITH node_agg AS (
    SELECT
        parent.id,
        parent.file_id,
        parent.node_type,
        parent.title,
        parent.subtype,
        parent.url,
        parent.icon,
        parent.created,
        parent.modified,
        parent.original_id,
        array_remove(array_agg(child.id), NULL) AS children
    FROM
        node AS parent
        LEFT JOIN node AS child ON child.parent_id = parent.id
    GROUP BY
        parent.id
)
SELECT
    file.path AS file_path,
    node_agg.*
FROM
    node_agg
    LEFT JOIN file ON node_agg.file_id = file.id;
