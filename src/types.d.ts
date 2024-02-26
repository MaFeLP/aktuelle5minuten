/**
 * The article and all its properties as extracted from the Deutschlandfunk website.
 */
type DlfArticle = {
    author: string,
    content: {
        html: string,
        plaintext: string,
    },
    date: string,
    description: string,
    figures: {
        caption: string,
        image: {
            src: string,
            alt: string,
            srcset: string,
            title: string,
        },
    }[],
    key: string,
    kicker: string,
    localeDate: string,
    title: string,
};

/**
 * Contents from the category endpoint, displays all the articles and contents,
 * as well as the requested category name.
 */
type PrintCategory = {
    category: string,
    text: string,
};

/**
 * The response from the /chatgpt endpoint
 */
type ChatGPTEnabled = {
    /**
     * Whether the ChatGPT button should be displayed
     */
    enabled: boolean,
};

type Progress = {
    articles: number,
    categories: number,
};
