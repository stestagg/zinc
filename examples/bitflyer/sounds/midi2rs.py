import os
import bisect
import collections
import midi
import click

def get_track(file):
    for track in file:
        if any(isinstance(e, midi.NoteOnEvent) for e in track):
            return track

ActiveNote = collections.namedtuple("ActiveNote", ("tick", "pitch"))
Note = collections.namedtuple("Note", ["start_tick", "pitch", "end_tick"])
Note.duration = property(lambda x: x.end_tick - x.start_tick)

Play = collections.namedtuple("Play", ["pitch_a", "pitch_b", "duration"])


def pad_pauses(notes):
    out = []
    last = 0
    for note in notes:
        if note.start_tick > last:
            out.append(Note(last, 0, note.start_tick))
        out.append(note)
        last = note.end_tick
    return out

def lstrip_pauses(plays):
    i = 0
    while plays[i].pitch_a == 0 and plays[i].pitch_b == 0:
        i += 1
    return plays[i:]

def promote_to_a(plays):
    for play in plays:
        if play.pitch_a == 0 and play.pitch_b != 0:
            yield Play(play.pitch_b, play.pitch_a, play.duration)
        else:
            yield play

def fix_pauses(plays, ave_len):
    min_pause = int(ave_len * 0.2)
    for play in plays:
        if play.pitch_a == 0 and play.pitch_b == 0 and play.duration < min_pause:
            yield Play(0, 0, min_pause)
        else:
            yield play


@click.command()
@click.argument("files", nargs=-1)
def main(files):
    for filename in files:
        basename, _ = os.path.splitext(filename)
        file = midi.read_midifile(filename)

        # tempo_changes = [(-1, 5000)]
        # for track in file:
        #   cur_tick = 0
        #   for event in track:
        #       cur_tick += event.tick
        #       if isinstance(event, midi.SetTempoEvent):
        #           tempo_changes.append((cur_tick, event.bpm))

        # tempo_indexes, tempos = zip(*tempo_changes)

        track = get_track(file)

        notes_a = []
        notes_b = []

        cur_note_a = None
        cur_note_b = None
        cur_tick = 0
        for event in track:
            cur_tick += event.tick
            if isinstance(event, midi.NoteOnEvent):
                pitch, vel = event.data
                if cur_note_a is None:
                    cur_note_a = ActiveNote(cur_tick, pitch)
                elif cur_note_b is None:
                    cur_note_b = ActiveNote(cur_tick, pitch)
                elif abs(cur_tick - cur_note_a.tick) < 20:
                    if pitch > cur_note_a.pitch:
                        cur_note_a = ActiveNote(cur_tick, pitch)
                elif abs(cur_tick - cur_note_b.tick) < 20:
                    if pitch > cur_note_b.pitch:
                        cur_note_b = ActiveNote(cur_tick, pitch)
            elif isinstance(event, midi.NoteOffEvent):
                pitch, _ = event.data
                if cur_note_a and pitch == cur_note_a.pitch:
                    notes_a.append(Note(cur_note_a.tick, cur_note_a.pitch, cur_tick))
                    cur_note_a = None
                if cur_note_b and pitch == cur_note_b.pitch:
                    notes_b.append(Note(cur_note_b.tick, cur_note_b.pitch, cur_tick))
                    cur_note_b = None

        notes_a = pad_pauses(notes_a)
        notes_b = pad_pauses(notes_b)

        plays = []

        while notes_a or notes_b:
            if not notes_a:
                next_b = notes_b.pop(0)
                plays.append(Play(0, next_b.pitch, next_b.duration))
            elif not notes_b:
                next_a = notes_a.pop(0)
                plays.append(Play(next_a.pitch, 0, next_a.duration))
            else:
                next_a = notes_a.pop(0)
                next_b = notes_b.pop(0)
                assert next_a.start_tick == next_b.start_tick
                end_tick = min(next_a.end_tick, next_b.end_tick)
                plays.append(Play(next_a.pitch, next_b.pitch, end_tick - next_a.start_tick))
                if next_a.end_tick > end_tick:
                    notes_a.insert(0, Note(end_tick, next_a.pitch, next_a.end_tick))
                if next_b.end_tick > end_tick:
                    notes_b.insert(0, Note(end_tick, next_b.pitch, next_b.end_tick))

        promoted = list(promote_to_a(lstrip_pauses(plays)))
        note_lengths = [p.duration for p in promoted if p.pitch_a or p.pitch_b]
        ave_note_len = sum(note_lengths) / len(note_lengths)
        out = [tuple(p) for p in fix_pauses(promoted, ave_note_len)]
        out.insert(0, (0,0,0))

        print "pub const %s: [(u16,u16,u16);%s] = [" % (basename.upper(), len(out))
        LINE_ITEMS = 10
        for i in range(0, len(out), LINE_ITEMS):
            print ", ".join(repr(r) for r in out[i:i+LINE_ITEMS]) + ","
        print "];"
        print


if __name__ == '__main__':
    main()