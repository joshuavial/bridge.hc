import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

import {
  AppAgentWebsocket,
  AppAgentClient,
} from '@holochain/client';
import { provide } from '@lit-labs/context';

import { AsyncStatus, StoreSubscriber } from '@holochain-open-dev/stores';

import {
  Profile,
  ProfilesClient,
  ProfilesStore,
} from '@holochain-open-dev/profiles';
import '@holochain-open-dev/profiles/elements/agent-avatar.js';
import '@holochain-open-dev/profiles/elements/profile-prompt.js';
import '@holochain-open-dev/profiles/elements/profile-list-item-skeleton.js';
import '@holochain-open-dev/elements/elements/display-error.js';
import "@holochain-open-dev/profiles/elements/profiles-context.js";
import { clientContext } from './contexts';

import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';

import "./components/header-component";
import "./components/footer-component";
import "./components/main-component";

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @state() loading = true;

  @state() profileCreated = false;

  @property()
  _profilesStore!: ProfilesStore;
  
  _myProfile!: StoreSubscriber<AsyncStatus<Profile | undefined>>;
  
  @provide({ context: clientContext })
  @property({ type: Object })
  _client!: AppAgentClient;
  
  async firstUpdated() {
    // We pass '' as url because it will dynamically be replaced in launcher environments

    this._client = await AppAgentWebsocket.connect('', 'bridge.hc');
    await this.initStores(this._client);

    const whoami = await this._client.callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "whoami",
      payload: null,
    });
    // TODO destructure this first? 
    const myProfile = await this._client.callZome({
      role_name: "bridge",
      zome_name: "profiles",
      fn_name: "get_agent_profile",
      payload: whoami
    });

    this.profileCreated = !!myProfile;
    
    this.loading = false;
  }

  async initStores(appAgentClient: AppAgentClient) {
    this._profilesStore = new ProfilesStore(
      new ProfilesClient(appAgentClient, 'bridge')
    );
    this._myProfile = new StoreSubscriber(
      this,
      () => this._profilesStore.myProfile
    );
    
  }

  renderMyProfile() {
    switch (this._myProfile.value.status) {
      case 'pending':
        return html`<profile-list-item-skeleton></profile-list-item-skeleton>`;
      case 'complete':
        if (!this._myProfile.value.value) return html``;

        return html`<div
          class="row"
          style="align-items: center;"
          slot="actionItems"
        >
          <agent-avatar .agentPubKey=${this._client.myPubKey}></agent-avatar>
          <span style="margin: 0 16px;">${this._myProfile.value.value?.nickname}</span>
        </div>`;
      case 'error':
        return html`<display-error
          .headline=${'Error fetching your profile'}
          .error=${this._myProfile.value.error.data.data}
          tooltip
        ></display-error>`;
      default:
        return html``
    }
  }

  render() {
    if (this.loading) return html`<sl-spinner></sl-spinner>`;

    if (!this.profileCreated) return html`
      <profiles-context .store="${this._profilesStore}">
        <create-profile @profile-created=${() => {
          this.profileCreated = true;
        }}>${this.renderMyProfile()}</create-profile>
      </profiles-context>
    `;

    return html`
      <header-component><div>${this.renderMyProfile()}</div></header-component>
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
