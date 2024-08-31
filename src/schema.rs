// @generated automatically by Diesel CLI.

diesel::table! {
    articles (key) {
        key -> Text,
        title -> Text,
        teaser_headline -> Text,
        teaser_text -> Text,
        date -> Timestamp,
        locale_date -> Text,
        kicker -> Nullable<Text>,
        description -> Nullable<Text>,
        content -> Nullable<Text>,
        category -> Nullable<Text>,
        status -> Integer,
        content_html -> Nullable<Text>,
        figure_src -> Nullable<Text>,
        figure_alt -> Nullable<Text>,
        figure_srcset -> Nullable<Text>,
        figure_title -> Nullable<Text>,
        figure_caption -> Nullable<Text>,
    }
}

diesel::table! {
    print_articles (id) {
        id -> Nullable<Integer>,
        category -> Text,
        bullets -> Text,
        created_at -> Nullable<Timestamp>,
        printed -> Nullable<Bool>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(articles, print_articles,);
