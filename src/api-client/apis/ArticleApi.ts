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


import * as runtime from '../runtime';
import type {
  Article,
} from '../models/index';
import {
    ArticleFromJSON,
    ArticleToJSON,
} from '../models/index';

export interface DemoteRequest {
    key?: string;
}

export interface GetRequest {
    key?: string;
}

export interface GetFirstRequest {
    articleDate?: string;
}

export interface PromoteRequest {
    category?: string;
    key?: string;
}

/**
 * 
 */
export class ArticleApi extends runtime.BaseAPI {

    /**
     * Sets the article\'s status to be **not** included in the PDF
     */
    async demoteRaw(requestParameters: DemoteRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        if (requestParameters['key'] != null) {
            queryParameters['key'] = requestParameters['key'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/article/demote`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Sets the article\'s status to be **not** included in the PDF
     */
    async demote(requestParameters: DemoteRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.demoteRaw(requestParameters, initOverrides);
    }

    /**
     * Retrieves a specific article from the database
     */
    async getRaw(requestParameters: GetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Article>> {
        const queryParameters: any = {};

        if (requestParameters['key'] != null) {
            queryParameters['key'] = requestParameters['key'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/article/get`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ArticleFromJSON(jsonValue));
    }

    /**
     * Retrieves a specific article from the database
     */
    async get(requestParameters: GetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Article> {
        const response = await this.getRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Gets all dates that have uncategorized articles
     */
    async getDatesRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<string>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/article/dates`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse<any>(response);
    }

    /**
     * Gets all dates that have uncategorized articles
     */
    async getDates(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<string>> {
        const response = await this.getDatesRaw(initOverrides);
        return await response.value();
    }

    /**
     * Gets the first article in the database
     */
    async getFirstRaw(requestParameters: GetFirstRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Article>> {
        const queryParameters: any = {};

        if (requestParameters['articleDate'] != null) {
            queryParameters['articleDate'] = requestParameters['articleDate'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/article/get/first`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ArticleFromJSON(jsonValue));
    }

    /**
     * Gets the first article in the database
     */
    async getFirst(requestParameters: GetFirstRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Article> {
        const response = await this.getFirstRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sets the article\'s status to be included in the PDF
     */
    async promoteRaw(requestParameters: PromoteRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        if (requestParameters['category'] != null) {
            queryParameters['category'] = requestParameters['category'];
        }

        if (requestParameters['key'] != null) {
            queryParameters['key'] = requestParameters['key'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/article/promote`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Sets the article\'s status to be included in the PDF
     */
    async promote(requestParameters: PromoteRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.promoteRaw(requestParameters, initOverrides);
    }

}
