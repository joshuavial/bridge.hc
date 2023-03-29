import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import {
  AppAgentWebsocket,
  ActionHash,
  AppAgentClient,
  AgentPubKey,
} from '@holochain/client';
import { provide } from '@lit-labs/context';

import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';

import "./components/header-component";
import "./components/footer-component";
import "./components/main-component";

import { Task } from '@lit-labs/task';
import { clientContext } from './contexts';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @state() loading = true;

  @provide({ context: clientContext })
  @property({ type: Object })
  client!: AppAgentClient;

  _init = new Task(this, () => this.client.callZome({
    cap_secret: null,
    role_name: 'bridge',
    zome_name: 'bridge',
    fn_name: 'wohami',
    payload: null,
}) as Promise<AgentPubKey>, () => []);

  async firstUpdated() {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    this.client = await AppAgentWebsocket.connect('', 'bridge.hc');

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
      margin: 0 auto;
      text-align: center;
      background-color: #fefefe;
    }
    :host > * {
      width: 100%;
      padding: 0;
      margin: 0;
    }
  `;
}
