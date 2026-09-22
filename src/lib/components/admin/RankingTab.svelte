<script lang="ts">
  import { onMount } from "svelte";
  import { getRanking, type RankingResponse } from "$lib/services/ranking";
  import { POINTS } from "$lib/config/points";
  import Button from "$lib/components/Button.svelte";
  import Icon from "$lib/components/Icon.svelte";

  let ranking = $state<RankingResponse | null>(null);
  let error = $state<string | undefined>(undefined);
  let loading = $state(true);

  async function load() {
    loading = true;
    error = undefined;
    try {
      ranking = await getRanking();
    } catch (err) {
      error = String(err);
    } finally {
      loading = false;
    }
  }

  onMount(load);
</script>

<section>
  <div class="header">
    <h2>Classement</h2>
    <div class="header-actions">
      {#if ranking}
        <p class="total">{ranking.totalVotes} vote{ranking.totalVotes === 1 ? "" : "s"} au total</p>
      {/if}
      <Button variant="secondary" icon="refresh" onclick={load} disabled={loading}>Actualiser</Button>
    </div>
  </div>

  {#if loading}
    <p>Chargement...</p>
  {:else if error}
    <p class="error" role="alert">
      <Icon name="alert-circle" size={20} />
      {error}
    </p>
  {:else if ranking && ranking.artists.length === 0}
    <p>Aucun artiste enregistré pour le moment.</p>
  {:else if ranking}
    <div class="table-wrapper animate-in">
      <table>
        <thead>
          <tr>
            <th scope="col">Rang</th>
            <th scope="col">Numéro</th>
            <th scope="col">Nom</th>
            <th scope="col">Points</th>
            <th scope="col">Top 1 ({POINTS.top1} pts)</th>
            <th scope="col">Top 2 ({POINTS.top2} pts)</th>
            <th scope="col">Top 3 ({POINTS.top3} pt)</th>
          </tr>
        </thead>
        <tbody>
          {#each ranking.artists as artist (artist.id)}
            <tr class:podium={artist.rank <= 3}>
              <td>
                <span class="rank" class:gold={artist.rank === 1} class:silver={artist.rank === 2} class:bronze={artist.rank === 3}>
                  {#if artist.rank <= 3}
                    <Icon name="trophy" size={16} />
                  {/if}
                  {artist.rank}
                </span>
              </td>
              <td>#{artist.id}</td>
              <td>{artist.name}</td>
              <td class="score">{artist.score}</td>
              <td>{artist.top1Count}</td>
              <td>{artist.top2Count}</td>
              <td>{artist.top3Count}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>

<style lang="scss">
  @use "../../styles/variables" as *;

  .header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: $space-3;
    margin-bottom: $space-4;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: $space-4;
  }

  .total {
    margin: 0;
    color: $color-text-muted;
  }

  .table-wrapper {
    overflow-x: auto;
    border-radius: $radius;
    border: 1px solid $color-border;
    box-shadow: $shadow-card;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    background-color: $color-surface;
  }

  th,
  td {
    text-align: left;
    padding: $space-3 $space-4;
    border-bottom: 1px solid $color-border;
  }

  th {
    background-color: $color-background;
    font-weight: 600;
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  tbody tr:hover td {
    background-color: $color-background;
  }

  .podium td {
    font-weight: 600;
  }

  .rank {
    display: inline-flex;
    align-items: center;
    gap: $space-1;
  }

  .gold {
    color: $color-gold;
  }

  .silver {
    color: $color-text-muted;
  }

  .bronze {
    color: #9c5a2e;
  }

  .score {
    font-weight: 700;
    color: $color-primary;
  }

  .error {
    display: flex;
    align-items: center;
    gap: $space-2;
    color: $color-danger;
  }
</style>
