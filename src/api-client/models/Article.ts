/* tslint:disable */
/* eslint-disable */
/**
 * Aktuelle 5 Minuten
 * Aktuelle 5 Minuten API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { ArticleContent } from './ArticleContent';
import {
    ArticleContentFromJSON,
    ArticleContentFromJSONTyped,
    ArticleContentToJSON,
} from './ArticleContent';
import type { ArticleFiguresInner } from './ArticleFiguresInner';
import {
    ArticleFiguresInnerFromJSON,
    ArticleFiguresInnerFromJSONTyped,
    ArticleFiguresInnerToJSON,
} from './ArticleFiguresInner';

/**
 * The article object
 * @export
 * @interface Article
 */
export interface Article {
    /**
     * 
     * @type {string}
     * @memberof Article
     */
    kicker: string;
    /**
     * 
     * @type {string}
     * @memberof Article
     */
    title: string;
    /**
     * 
     * @type {string}
     * @memberof Article
     */
    description: string;
    /**
     * 
     * @type {ArticleContent}
     * @memberof Article
     */
    content: ArticleContent;
    /**
     * 
     * @type {Array<ArticleFiguresInner>}
     * @memberof Article
     */
    figures: Array<ArticleFiguresInner>;
    /**
     * 
     * @type {string}
     * @memberof Article
     */
    key: string;
    /**
     * 
     * @type {string}
     * @memberof Article
     */
    date: string;
    /**
     * 
     * @type {string}
     * @memberof Article
     */
    localeDate: string;
}

/**
 * Check if a given object implements the Article interface.
 */
export function instanceOfArticle(value: object): boolean {
    if (!('kicker' in value)) return false;
    if (!('title' in value)) return false;
    if (!('description' in value)) return false;
    if (!('content' in value)) return false;
    if (!('figures' in value)) return false;
    if (!('key' in value)) return false;
    if (!('date' in value)) return false;
    if (!('localeDate' in value)) return false;
    return true;
}

export function ArticleFromJSON(json: any): Article {
    return ArticleFromJSONTyped(json, false);
}

export function ArticleFromJSONTyped(json: any, ignoreDiscriminator: boolean): Article {
    if (json == null) {
        return json;
    }
    return {
        
        'kicker': json['kicker'],
        'title': json['title'],
        'description': json['description'],
        'content': ArticleContentFromJSON(json['content']),
        'figures': ((json['figures'] as Array<any>).map(ArticleFiguresInnerFromJSON)),
        'key': json['key'],
        'date': json['date'],
        'localeDate': json['localeDate'],
    };
}

export function ArticleToJSON(value?: Article | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'kicker': value['kicker'],
        'title': value['title'],
        'description': value['description'],
        'content': ArticleContentToJSON(value['content']),
        'figures': ((value['figures'] as Array<any>).map(ArticleFiguresInnerToJSON)),
        'key': value['key'],
        'date': value['date'],
        'localeDate': value['localeDate'],
    };
}

