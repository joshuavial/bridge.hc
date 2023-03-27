import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import {
  AppAgentWebsocket,
  ActionHash,
  AppAgentClient,
} from '@holochain/client';
import { provide } from '@lit-labs/context';

import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';

import "./components/header-component";
import "./components/footer-component";
import "./components/main-component";

import { clientContext } from './contexts';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @state() loading = true;

  @provide({ context: clientContext })
  @property({ type: Object })
  client!: AppAgentClient;

  async firstUpdated() {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    // this.client = await AppAgentWebsocket.connect('', 'bridge.hc');

    this.loading = false;
  }

  render() {
    if (this.loading)
      return html`
      <sl-spinner></sl-spinner>
      `;

    return html`
      <header-component></header-component>
      <main-component></main-component>
      <footer-component></footer-component>
    `;
  }

  static styles = css`
    :host {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
      font-size: calc(10px + 2vmin);
      font-family: "Roboto", sans-serif;
      color: #1a2b42;
      max-width: 960px;
      margin: 0 auto;
      text-align: center;
      // background-color: var(--lit-element-background-color);
      background-color: #fefefe;
    }
  `;
}
